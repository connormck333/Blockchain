use libp2p::{noise, ping, tcp, yamux};
use tracing_subscriber::EnvFilter;


mod block;
mod transaction;
mod blockchain;
mod utils;
mod wallet;
mod network;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default
        )?
        .with_behaviour(|_| ping::Behaviour::default())?
        .build();

    Ok(())
}