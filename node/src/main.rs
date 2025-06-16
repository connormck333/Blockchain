use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use clap::Parser;
use anyhow::Result;
use args::args::Args;
use crate::args::node_type::NodeType;
use crate::server::server::start_server;
use crate::database::connection::Connection;
use crate::database::validator::Validator;
use crate::mining_tasks::spawn_mining_loop;
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

    let node = create_node(&args);
    let mining_flag = Arc::new(AtomicBool::new(true));
    let mempool = node.lock().await.mempool.clone();
    let db_connection = Arc::new(Connection::new(node.lock().await.id).await);
    let validator = Arc::new(Validator::new(db_connection.clone()));

    let wallet = node.lock().await.wallet.clone();
    println!("Wallet private key: {}", wallet.get_private_key());
    println!("Wallet public key: {}", wallet.get_public_key());
    println!("Wallet address: {}", wallet.address);

    let miner_address = node.lock().await.wallet.address.clone();
    db_connection.create_user(miner_address.clone(), 0).await;

    println!("Mining genesis block");
    let genesis_block = node.lock().await.blockchain.create_genesis_block(miner_address.clone());

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

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    db_connection.drop_database().await;
    println!("Database dropped successfully.");
    Ok(())
}