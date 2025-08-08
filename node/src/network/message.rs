use anyhow::Result;
use iroh::NodeId;
use serde::{Deserialize, Serialize};
use crate::block::Block;
use crate::transaction::Transaction;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    PeerConnectionRequest { from: String },
    PeerConnectionResponse { from: String, known_addresses: Vec<String> },
    BlockMined { from: String, block: Block },
    TransactionCreated { from: NodeId, transaction: Transaction },
    GenesisBlock { from: String, genesis_block: Block },
    FullChainRequest { from: String },
    FullChainResponse { from: String, blocks: Vec<Block> },
    ChainLengthRequest { from: String },
    ChainLengthResponse { from: String, length: usize },
    BlockHashesRequest { from: String, hashes: Vec<String> },
    BlockHashesResponse { from: String, hashes: Vec<String>, common_index: usize },
    GetBlocks { from: String, hashes: Vec<String> },
    BlockList { from: String, blocks: Vec<Block> }
}

#[derive(Clone)]
pub struct ChainLength {
    pub from: String,
    pub length: usize
}

impl Message {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(Into::into)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Failed to serialize message")
    }
}