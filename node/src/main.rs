use std::sync::Arc;
use clap::Parser;
use anyhow::Result;
use tokio::sync::Mutex;
use args::args::Args;
use crate::args::node_type::NodeType;
use crate::network::network::Network;
use crate::network::node::Node;
use crate::server::server::start_server;
use crate::database::connection::Connection;
use crate::database::validator::Validator;

extern crate sqlx;

mod block;
mod transaction;
mod blockchain;
mod utils;
mod wallet;
mod network;
mod server;
mod args;
mod database;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let args = Args::parse();
    let node_name: String = if args.name.is_some() {args.clone().name.unwrap()} else {"".to_string()};

    let mut network = Network::new(args.clone());
    let node = Arc::new(Mutex::new(Node::new(&node_name, 5)));
    let mempool = node.lock().await.mempool.clone();
    let db_connection = Arc::new(Connection::new(node.lock().await.id).await);
    let validator = Arc::new(Validator::new(db_connection.clone()));

    network.connect(node.clone(), validator.clone()).await?;

    if args.node_type == NodeType::FULL {
        tokio::select! {
            _ = start_server(mempool.clone(), validator.clone()) => {
                println!("Server shutting down...");
            }
            _ = tokio::signal::ctrl_c() => {
                println!("\nCtrl+C received, cleaning up...");
                cleanup(db_connection.clone()).await.expect("Cleanup failed");
            }
        }
    } else {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                println!("\nCtrl+C received, cleaning up...");
                cleanup(db_connection.clone()).await.expect("Cleanup failed");
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