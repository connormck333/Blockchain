use std::fmt::Display;
use chrono::Utc;
use hex::encode;
use serde::{Serialize, Deserialize};
use serde_json::to_string;
use sha2::{Digest, Sha256};
use crate::constants::BLOCKCHAIN_DIFFICULTY;
use crate::chain::transaction::Transaction;

/*
    Each block in the blockchain is a digital container
    that permanently stores transaction data for the network.
*/

#[derive(Serialize, Deserialize)]
struct HashlessBlock {
    index: u64,
    timestamp: i64,
    transactions: Vec<Transaction>,
    miner_address: String,
    previous_block_hash: String,
    nonce: u64,
    difficulty: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_block_hash: String,
    pub miner_address: String,
    pub nonce: u64,
    pub difficulty: usize,
    pub hash: String
}

impl Block {
    pub fn new(index: u64, previous_block_hash: String, transactions: Vec<Transaction>, miner_address: String) -> Self {
        Self {
            index,
            timestamp: Utc::now().timestamp(),
            transactions,
            previous_block_hash,
            miner_address,
            nonce: 0,
            difficulty: BLOCKCHAIN_DIFFICULTY,
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
            miner_address: self.miner_address.clone(),
            nonce: self.nonce,
            difficulty: self.difficulty
        };

        let serialized: String = to_string(&hashless_block).expect("Failed to serialize block");

        Self::calculate_hash(serialized)
    }

    pub fn calculate_hash(serialized_data: String) -> String {
        let mut hasher = Sha256::new();
        hasher.update(serialized_data.as_bytes());
        let result = hasher.finalize();

        encode(result)
    }

    pub fn equals(&self, other: &Block) -> bool {
        self.index == other.index &&
        self.timestamp == other.timestamp &&
        self.transactions == other.transactions &&
        self.previous_block_hash == other.previous_block_hash &&
        self.miner_address == other.miner_address &&
        self.nonce == other.nonce &&
        self.difficulty == other.difficulty &&
        self.hash == other.hash
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
        let new_block = Block::new(0, "previousBlockHash".to_string(), vec![], "minerAddress".to_string());

        assert_eq!(new_block.difficulty, 0);
        assert!(new_block.transactions.is_empty());
        assert_eq!(new_block.index, 0);
        assert_eq!(new_block.previous_block_hash, "previousBlockHash".to_string());
        assert_eq!(new_block.miner_address, "minerAddress".to_string());
        assert_eq!(new_block.hash, "");
        assert_eq!(new_block.nonce, 0);
    }

    #[test]
    fn test_mine() {
        let mut new_block = Block::new(0, "previousBlockHash".to_string(), vec![], "minerAddress".to_string());

        new_block.mine();

        assert_eq!(new_block.hash, new_block.create_hash());
        assert!(new_block.hash.starts_with(&"0".repeat(2)));
        assert!(new_block.nonce > 0);
    }
}