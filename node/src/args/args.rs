use clap::{Parser};
use crate::args::node_type::NodeType;

#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[clap(subcommand)]
    pub node_type: NodeType
}