use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;
use clap::Parser;
use anyhow::Result;
use tokio::sync::Mutex;
use args::args::Args;
use crate::args::mode::Mode;
use crate::args::node_type::NodeType;
use crate::server::server::start_server;
use crate::database::connection::Connection;
use crate::database::validator::Validator;
use crate::mining_tasks::spawn_mining_loop;
use crate::network::message_sender::MessageSender;
use crate::network::tcp_connection::create_node;

extern crate sqlx;

mod block;
mod transaction;
mod blockchain;
mod mining_tasks;
mod wallet;
mod network;
mod server;
mod args;
mod database;
mod mining_reward;
mod constants;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let args = Args::parse();
    let is_opening_node = matches!(args.node_type.get_mode(), Mode::OPEN { .. });

    let mining_flag = Arc::new(AtomicBool::new(true));
    let db_connection = Arc::new(Connection::new().await);
    let validator = Arc::new(Validator::new(db_connection.clone()));
    let node = create_node(&args, validator.clone(), mining_flag.clone());
    let message_sender = Arc::new(Mutex::new(MessageSender::new(node.clone())));
    let mempool = node.lock().await.mempool.clone();

    let wallet = node.lock().await.wallet.clone();
    println!("Wallet private key: {}", wallet.get_private_key());
    println!("Wallet public key: {}", wallet.get_public_key());
    println!("Wallet address: {}", wallet.address);

    let miner_address = node.lock().await.wallet.address.clone();
    db_connection.create_user(miner_address.clone(), 0).await;

    println!("Mining genesis block");
    let genesis_block = node.lock().await.blockchain.create_genesis_block(miner_address.clone());

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

    if is_opening_node {
        message_sender.lock().await.send_genesis_block(&genesis_block).await;
    }

    println!("Starting mining...");
    spawn_mining_loop(node.clone(), mining_flag.clone(), db_connection.clone());
    println!("Mining has commenced");

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


async fn cleanup(db_connection: Arc<Connection>) -> Result<()> {
    let pool = db_connection.pool.clone();
    drop(pool);

    tokio::time::sleep(Duration::from_millis(200)).await;

    db_connection.drop_database().await;
    println!("Database dropped successfully.");
    Ok(())
}