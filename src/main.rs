use std::sync::Arc;
use clap::Parser;
use anyhow::Result;
use tokio::sync::Mutex;
use crate::network::args::Args;
use crate::network::network::Network;
use crate::network::node::Node;

mod block;
mod transaction;
mod blockchain;
mod utils;
mod wallet;
mod network;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let node_name: String = if args.name.is_some() {args.clone().name.unwrap()} else {"".to_string()};

    let mut network = Network::new(args);
    let node = Arc::new(Mutex::new(Node::new(&node_name, 5)));

    network.connect(node).await?;

    tokio::signal::ctrl_c().await?;
    println!("Shutting down...");

    Ok(())
}