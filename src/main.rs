use clap::Parser;
use anyhow::Result;
use crate::network::args::Args;
use crate::network::network::Network;

mod block;
mod transaction;
mod blockchain;
mod utils;
mod wallet;
mod network;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut network = Network::new(args);

    network.connect().await
}