use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
pub struct ModeArgs {
    #[command(subcommand)]
    pub mode: Mode
}

#[derive(Subcommand, Debug, Clone)]
pub enum Mode {
    OPEN {
        node_address: String,
        external_address: String
    },
    JOIN {
        node_address: String,
        peer_address: String,
        external_address: String
    }
}