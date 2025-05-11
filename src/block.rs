use std::fmt::Display;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use serde_json::to_string;
use crate::transaction::Transaction;
use crate::utils::calculate_hash;

/*
    Each block in the blockchain is a digital container
    that permanently stores transaction data for the network.
*/

#[derive(Serialize, Deserialize)]
struct HashlessBlock {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_block_hash: String,
    pub nonce: u64,
    pub difficulty: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_block_hash: String,
    pub nonce: u64,
    pub difficulty: usize,
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

    pub fn mine(&mut self) -> bool {
        let hash: String = self.create_hash();

        if hash.starts_with(&"0".repeat(self.difficulty)) {
            self.hash = hash;
            return true;
        }

        self.nonce += 1;

        false
    }

    pub fn create_hash(&self) -> String {
        let hashless_block = HashlessBlock {
            index: self.index,
            timestamp: self.timestamp,
            transactions: self.transactions.clone(),
            previous_block_hash: self.previous_block_hash.clone(),
            nonce: self.nonce,
            difficulty: self.difficulty
        };

        let serialized: String = to_string(&hashless_block).expect("Failed to serialize block");

        calculate_hash(serialized)
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "index: {}, timestamp, {}, transactions: {}, prev block hash: {}, nonce: {}, difficulty: {}, hash: {}",
               self.index, self.timestamp, self.transactions.len(), self.previous_block_hash, self.nonce, self.difficulty, self.hash)
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