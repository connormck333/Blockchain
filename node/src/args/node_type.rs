use clap::ValueEnum;

#[derive(ValueEnum, Debug, Clone, PartialEq)]
pub enum NodeType {
    MINER,
    FULL
}