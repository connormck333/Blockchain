use clap::Parser;
use anyhow::Result;
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
    let name: String = if args.name.is_some() {args.clone().name.unwrap()} else {"".to_string()};

    let mut network = Network::new(args.clone());
    let mut node = Node::new(&name, 3);

    network.connect().await
}