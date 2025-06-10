use clap::Subcommand;
use crate::args::mode::{Mode, ModeArgs};

#[derive(Subcommand, Debug, Clone)]
pub enum NodeType {
    MINER(ModeArgs),
    FULL(ModeArgs)
}

impl NodeType {
    pub fn get_mode(&self) -> &Mode {
        match self {
            NodeType::MINER(args) => &args.mode,
            NodeType::FULL(args) => &args.mode
        }
    }
}