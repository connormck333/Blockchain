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
use crate::database::operations::DbOperations;
use crate::database::validator::Validator;
use crate::mining_tasks::spawn_mining_loop;
use crate::network::message::Message;
use crate::network::message_sender::broadcast_message;
use crate::network::node::Node;
use crate::network::tcp_connection::{create_node, start_peer_connection};
use crate::server::server::start_server;

extern crate sqlx;

pub async fn test_init(
    node: Arc<Mutex<Node>>,
    db: DbOperations,
    args: Args
) -> Result<()> {
    let mining_flag = Arc::new(AtomicBool::new(true));
    let validator = Arc::new(Validator::new(db.clone()));

    let miner_address = node.lock().await.wallet.address.clone();
    let peer_address = match args.node_type.get_mode() {
        Mode::OPEN { .. } => None,
        Mode::JOIN { peer_address, .. } => Some(peer_address.clone()),
    };

    start_peer_connection(node.clone(), validator.clone(), mining_flag.clone(), peer_address).await;

    start_blockchain(
        mining_flag.clone(),
        node.clone(),
        db.clone(),
        miner_address,
        args
    ).await
}

pub async fn init() -> Result<()> {
    let args = Args::parse();
    let mining_flag = Arc::new(AtomicBool::new(true));
    let db = Arc::new(Connection::new().await);
    let validator = Arc::new(Validator::new(db.clone()));
    let node = create_node(&args, validator.clone(), mining_flag.clone()).await;
    let mempool = node.lock().await.mempool.clone();

    let wallet = node.lock().await.wallet.clone();
    println!("Wallet private key: {}", wallet.get_private_key());
    println!("Wallet public key: {}", wallet.get_public_key());
    println!("Wallet address: {}", wallet.address);

    let miner_address = node.lock().await.wallet.address.clone();
    db.create_user(miner_address.clone(), 0).await;

    start_blockchain(
        mining_flag.clone(),
        node.clone(),
        db.clone(),
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
                    cleanup(db.clone()).await.expect("Cleanup failed");
                }
            }
        }
        _ => {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    println!("\nCtrl+C received, cleaning up...");
                    cleanup(db.clone()).await.expect("Cleanup failed");
                }
            }
        }
    }

    Ok(())
}

async fn start_blockchain(
    mining_flag: Arc<AtomicBool>,
    node: Arc<Mutex<Node>>,
    db: DbOperations,
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
        broadcast_message(node.clone(), &genesis_message).await;
    }

    spawn_mining_loop(node.clone(), mining_flag.clone(), db.clone());

    Ok(())
}


pub async fn cleanup(db: DbOperations) -> Result<()> {
    let pool = db.get_pool().clone();
    drop(pool);

    tokio::time::sleep(Duration::from_millis(200)).await;

    db.drop_database().await;
    println!("Database dropped successfully.");
    Ok(())
}