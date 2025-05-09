use chrono::Utc;
use serde::{Serialize, Deserialize};
use serde_json::to_string;
use crate::transaction::Transaction;
use crate::utils::calculate_hash;

/*
    Each block in the blockchain is a digital container
    that permanently stores transaction data for the network.
*/

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_block_hash: String,
    pub nonce: u64,
    pub difficulty: usize,

    #[serde(skip_serializing, skip_deserializing)]
    pub hash: String
}

impl Block {
    pub fn new(index: u64, previous_block_hash: String, transactions: Vec<Transaction>, difficulty: usize) -> Self {
        Self {
            index,
            timestamp: Utc::now().timestamp(),
            transactions,
            previous_block_hash,
            nonce: 0,
            difficulty,
            hash: String::new()
        }
    }

    pub fn mine(&mut self) {
        loop {
            let hash: String = self.create_hash();

            if hash.starts_with(&"0".repeat(self.difficulty)) {
                self.hash = hash;
                break;
            }

            self.nonce += 1;
        }
    }

    pub fn create_hash(&self) -> String {
        let serialized: String = to_string(self).expect("Failed to serialize block");

        calculate_hash(serialized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_constructor() {
        let new_block = Block::new(0, "previousBlockHash".to_string(), vec![], 0);

        assert_eq!(new_block.difficulty, 0);
        assert!(new_block.transactions.is_empty());
        assert_eq!(new_block.index, 0);
        assert_eq!(new_block.previous_block_hash, "previousBlockHash".to_string());
        assert_eq!(new_block.hash, "");
        assert_eq!(new_block.nonce, 0);
    }

    #[test]
    fn test_mine() {
        let mut new_block = Block::new(0, "previousBlockHash".to_string(), vec![], 3);

        new_block.mine();

        assert_eq!(new_block.hash, new_block.create_hash());
        assert!(new_block.hash.starts_with(&"0".repeat(2)));
        assert!(new_block.nonce > 0);
    }
}