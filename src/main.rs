use std::sync::Arc;
use clap::Parser;
use anyhow::Result;
use tokio::sync::Mutex;
use args::args::Args;
use crate::args::node_type::NodeType;
use crate::network::network::Network;
use crate::network::node::Node;
use crate::server::server::start_server;

mod block;
mod transaction;
mod blockchain;
mod utils;
mod wallet;
mod network;
mod server;
mod args;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let node_name: String = if args.name.is_some() {args.clone().name.unwrap()} else {"".to_string()};

    let mut network = Network::new(args.clone());
    let node = Arc::new(Mutex::new(Node::new(&node_name, 5)));

    let mempool = node.lock().await.mempool.clone();
    let wallet = node.lock().await.wallet.clone();

    network.connect(node.clone()).await?;

    if args.node_type == NodeType::FULL {
        start_server(mempool, wallet).await?;
    }

    tokio::signal::ctrl_c().await?;
    println!("Shutting down...");

    Ok(())
}