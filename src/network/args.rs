use clap::{Parser};
use crate::network::command::Command;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(short, long)]
    pub name: Option<String>,

    #[clap(short, long, default_value = "0")]
    pub bind_port: u16,

    #[clap(subcommand)]
    pub command: Command
}