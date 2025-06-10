use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
pub struct ModeArgs {
    #[command(subcommand)]
    pub mode: Mode
}

#[derive(Subcommand, Debug, Clone)]
pub enum Mode {
    OPEN,
    JOIN {
        ticket: String
    }
}