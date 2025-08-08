use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::block::Block;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    PeerConnectionRequest { from: String },
    PeerConnectionResponse { from: String, known_addresses: Vec<String> },
    BlockMined { from: String, block: Block },
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::Block;

    #[test]
    fn test_message_serialization_deserialization() {
        let block = Block {
            index: 1,
            previous_block_hash: "prev_hash".to_string(),
            hash: "hash".to_string(),
            timestamp: 123456789,
            transactions: vec![],
            miner_address: "miner1".to_string(),
            nonce: 42,
            difficulty: 0,
        };
        let msg = Message::BlockMined { from: "node1".to_string(), block: block.clone() };
        let serialized = serde_json::to_vec(&msg).unwrap();
        let deserialized: Message = Message::from_bytes(&serialized).unwrap();
        match deserialized {
            Message::BlockMined { from, block: b } => {
                assert_eq!(from, "node1");
                assert_eq!(b.index, block.index);
                assert_eq!(b.hash, block.hash);
            },
            _ => panic!("Deserialized to wrong variant"),
        }
    }

    #[test]
    fn test_chain_length_response_serialization() {
        let msg = Message::ChainLengthResponse { from: "node2".to_string(), length: 10 };
        let serialized = serde_json::to_vec(&msg).unwrap();
        let deserialized: Message = Message::from_bytes(&serialized).unwrap();
        match deserialized {
            Message::ChainLengthResponse { from, length } => {
                assert_eq!(from, "node2");
                assert_eq!(length, 10);
            },
            _ => panic!("Deserialized to wrong variant"),
        }
    }
}
