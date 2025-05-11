use anyhow::Result;
use iroh::NodeId;
use serde::{Deserialize, Serialize};
use crate::block::Block;
use crate::transaction::Transaction;

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    BlockMined { from: NodeId, block: Block },
    TransactionCreated { from: NodeId, transaction: Transaction },
    GenesisBlock { from: NodeId, genesis_block: Block }
}

impl Message {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(Into::into)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Failed to serialize message")
    }
}