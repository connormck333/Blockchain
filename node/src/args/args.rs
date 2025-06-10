use clap::{Parser};
use crate::args::node_type::NodeType;

#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[clap(short, long)]
    pub name: Option<String>,

    #[clap(short, long, default_value = "0")]
    pub bind_port: u16,

    #[clap(subcommand)]
    pub node_type: NodeType
}