use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use serde::{Serialize, Deserialize};
use serde_json::to_string;
use hex::encode;
use sha2::{Sha256, Digest};
use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_block_hash: String,
    pub nonce: u64,

    #[serde(skip_serializing)]
    pub hash: String
}

impl Block {
    pub fn create(index: u64, previous_block_hash: String, transactions: Vec<Transaction>) -> Block {
        let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let timestamp = since_the_epoch.as_secs() * 1000 +
            since_the_epoch.subsec_nanos() as u64 / 1_000_000;

        let new_nonce = rand::rng().random_range(0..10000);

        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_block_hash,
            nonce: new_nonce,
            hash: String::new()
        };

        block.hash = block.calculate_hash();

        block
    }

    pub fn calculate_hash(&self) -> String {
        let serialized = to_string(self).expect("Failed to serialize block");
        let mut hasher = Sha256::new();
        hasher.update(serialized.as_bytes());
        let result = hasher.finalize();

        encode(result)
    }
}