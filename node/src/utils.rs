use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use hex::encode;
use sha2::{Digest, Sha256};
use crate::block::Block;
use crate::transaction::Transaction;

pub fn calculate_hash(serialized_data: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(serialized_data.as_bytes());
    let result = hasher.finalize();

    encode(result)
}

pub fn mine_block(
    transactions: Vec<Transaction>,
    difficulty: usize,
    previous_hash: String,
    block_index: u64,
    cancel_flag: Arc<AtomicBool>,
    node_address: String
) -> Option<Block> {
    println!("-----> Transaction count picked up for mining: {}", transactions.len());
    let mut block = Block::new(block_index, previous_hash, transactions, difficulty, node_address);

    while cancel_flag.load(Ordering::Relaxed) == true {
        if block.mine() {
            println!("Mined block {}", block.index);
            return Some(block);
        }
    }

    None
}