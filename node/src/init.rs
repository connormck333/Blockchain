use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;
use clap::Parser;
use anyhow::Result;
use tokio::sync::Mutex;
use crate::args::args::Args;
use crate::args::mode::Mode;
use crate::args::node_type::NodeType;
use crate::block::Block;
use crate::database::connection::Connection;
use crate::database::validator::Validator;
use crate::mining_tasks::spawn_mining_loop;
use crate::network::message::Message;
use crate::network::message_sender::MessageSender;
use crate::network::node::Node;
use crate::network::tcp_connection::{create_node, start_peer_connection};
use crate::server::server::start_server;

extern crate sqlx;

pub async fn test_init(
    node: Arc<Mutex<Node>>,
    db_connection: Arc<Connection>,
    args: Args
) -> Result<()> {
    let mining_flag = Arc::new(AtomicBool::new(true));
    let validator = Arc::new(Validator::new(db_connection.clone()));
    let message_sender = Arc::new(Mutex::new(MessageSender::new(node.clone())));

    let miner_address = node.lock().await.wallet.address.clone();
    db_connection.create_user(miner_address.clone(), 0).await;

    let peer_address = match args.node_type.get_mode() {
        Mode::OPEN { .. } => None,
        Mode::JOIN { peer_address, .. } => Some(peer_address.clone()),
    };

    start_peer_connection(node.clone(), validator.clone(), mining_flag.clone(), peer_address).await;

    start_blockchain(
        mining_flag.clone(),
        node.clone(),
        db_connection.clone(),
        message_sender.clone(),
        miner_address,
        args
    ).await
}

pub async fn init() -> Result<()> {
    let args = Args::parse();
    let mining_flag = Arc::new(AtomicBool::new(true));
    let db_connection = Arc::new(Connection::new().await);
    let validator = Arc::new(Validator::new(db_connection.clone()));
    let node = create_node(&args, validator.clone(), mining_flag.clone()).await;
    let message_sender = Arc::new(Mutex::new(MessageSender::new(node.clone())));
    let mempool = node.lock().await.mempool.clone();

    let wallet = node.lock().await.wallet.clone();
    println!("Wallet private key: {}", wallet.get_private_key());
    println!("Wallet public key: {}", wallet.get_public_key());
    println!("Wallet address: {}", wallet.address);

    let miner_address = node.lock().await.wallet.address.clone();
    db_connection.create_user(miner_address.clone(), 0).await;

    start_blockchain(
        mining_flag.clone(),
        node.clone(),
        db_connection.clone(),
        message_sender.clone(),
        miner_address,
        args.clone()
    ).await?;

    match args.node_type {
        NodeType::FULL(_) => {
            tokio::select! {
                _ = start_server(mempool.clone(), validator.clone()) => {
                    println!("Server shutting down...");
                }
                _ = tokio::signal::ctrl_c() => {
                    println!("\nCtrl+C received, cleaning up...");
                    cleanup(db_connection.clone()).await.expect("Cleanup failed");
                }
            }
        }
        _ => {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    println!("\nCtrl+C received, cleaning up...");
                    cleanup(db_connection.clone()).await.expect("Cleanup failed");
                }
            }
        }
    }

    Ok(())
}

async fn start_blockchain(
    mining_flag: Arc<AtomicBool>,
    node: Arc<Mutex<Node>>,
    db_connection: Arc<Connection>,
    message_sender: Arc<Mutex<MessageSender>>,
    miner_address: String,
    args: Args
) -> Result<()> {
    let is_opening_node = matches!(args.node_type.get_mode(), Mode::OPEN { .. });

    let mut genesis_block: Option<Block> = None;
    if is_opening_node {
        println!("Mining genesis block");
        genesis_block = Some(node.lock().await.blockchain.create_genesis_block(miner_address.clone()));
    }

    loop {
        println!("Checking connection...");
        if !node.lock().await.peers.is_empty() {
            println!("Connected to {} peers", node.lock().await.peers.len());
            break;
        }

        println!("No peer connection established: {}.", node.lock().await.peers.len());
        println!("Checking connection again in 5 seconds");
        tokio::time::sleep(Duration::from_secs(5)).await;
    }

    if genesis_block.is_some() {
        let genesis_message = Message::GenesisBlock {
            from: node.lock().await.address.clone(),
            genesis_block: genesis_block.unwrap().clone()
        };
        message_sender.lock().await.broadcast_message(&genesis_message).await;
    }

    spawn_mining_loop(node.clone(), mining_flag.clone(), db_connection.clone(), message_sender.clone());

    Ok(())
}


pub async fn cleanup(db_connection: Arc<Connection>) -> Result<()> {
    let pool = db_connection.pool.clone();
    drop(pool);

    tokio::time::sleep(Duration::from_millis(200)).await;

    db_connection.drop_database().await;
    println!("Database dropped successfully.");
    Ok(())
}