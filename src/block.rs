use chrono::Utc;
use serde::{Serialize, Deserialize};
use serde_json::to_string;
use crate::transaction::Transaction;
use crate::utils::calculate_hash;

/*
    Each block in the blockchain is a digital container
    that permanently stores transaction data for the network.
*/

#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_block_hash: String,
    pub nonce: u64,
    pub difficulty: usize,

    #[serde(skip_serializing)]
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
            println!("hash: {}", hash);

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