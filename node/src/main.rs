use std::sync::{Arc, Mutex};
use clap::Parser;
use anyhow::Result;
use args::args::Args;
use crate::args::node_type::NodeType;
use crate::network::network::Network;
use crate::network::node::Node;
use crate::server::server::start_server;
use crate::database::connection::Connection;
use crate::database::validator::Validator;
use crate::p2p::client::Client;
use crate::p2p::tcp_connection::{connect_to_peer, start_client};

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
mod p2p;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client1 = Arc::new(Mutex::new(Client::new("127.0.0.1:8080".to_string())));
    let server1 = tokio::spawn(start_client(client1.clone()));
    let connection1 = tokio::spawn(connect_to_peer(client1.clone(), "127.0.0.1:8088"));

    let mut client2 = Arc::new(Mutex::new(Client::new("127.0.0.1:8088".to_string())));
    let server2 = tokio::spawn(start_client(client2.clone()));

    let _ = tokio::join!(server1, connection1, server2);


    // dotenv::dotenv().ok();
    //
    // let args = Args::parse();
    // let node_name: String = if args.name.is_some() {args.clone().name.unwrap()} else {"".to_string()};
    //
    // let mut network = Network::new(args.clone());
    // let node = Arc::new(Mutex::new(Node::new(&node_name, 5)));
    // let mempool = node.lock().await.mempool.clone();
    // let db_connection = Arc::new(Connection::new(node.lock().await.id).await);
    // let validator = Arc::new(Validator::new(db_connection.clone()));
    //
    // let wallet = node.lock().await.wallet.clone();
    // println!("Wallet private key: {}", wallet.get_private_key());
    // println!("Wallet public key: {}", wallet.get_public_key());
    // println!("Wallet address: {}", wallet.address);
    //
    // db_connection.create_user(node.lock().await.clone().wallet.address, 0).await;
    // network.connect(node.clone(), validator.clone()).await?;
    //
    // match args.node_type {
    //     NodeType::FULL(_) => {
    //         tokio::select! {
    //             _ = start_server(mempool.clone(), validator.clone()) => {
    //                 println!("Server shutting down...");
    //             }
    //             _ = tokio::signal::ctrl_c() => {
    //                 println!("\nCtrl+C received, cleaning up...");
    //                 cleanup(db_connection.clone()).await.expect("Cleanup failed");
    //             }
    //         }
    //     }
    //     _ => {
    //         tokio::select! {
    //             _ = tokio::signal::ctrl_c() => {
    //                 println!("\nCtrl+C received, cleaning up...");
    //                 cleanup(db_connection.clone()).await.expect("Cleanup failed");
    //             }
    //         }
    //     }
    // }

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