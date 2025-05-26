use clap::{Parser};
use crate::args::command::Command;
use crate::args::node_type::NodeType;

#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[clap(short, long)]
    pub name: Option<String>,

    #[clap(short, long, default_value = "0")]
    pub bind_port: u16,

    #[clap(subcommand)]
    pub command: Command,

    #[clap(long, value_enum, default_value_t = NodeType::FULL)]
    pub node_type: NodeType
}