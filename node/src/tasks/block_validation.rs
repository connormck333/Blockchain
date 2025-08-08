use std::collections::HashSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Mutex;
use crate::block::Block;
use crate::constants::{MINING_REWARD_AMOUNT, MINING_REWARD_DELAY};
use crate::database::operations::DbOperations;
use crate::database::validator::Validator;
use crate::mining_reward::MiningReward;
use crate::mining_tasks::spawn_update_balances;
use crate::network::message::Message;
use crate::network::message_sender::broadcast_message;
use crate::network::node::Node;
use crate::tasks::fork_handling::wait_and_send_block_hashes;

pub async fn on_valid_block_received(validator: Arc<Validator>, mining_flag: Arc<AtomicBool>, block: &Block, from: &str) {
    mining_flag.store(false, Ordering::Relaxed);
    println!("Valid block received from {}... Stopping mining", from);

    let mining_reward = MiningReward::new(MINING_REWARD_AMOUNT, block.miner_address.clone(), block.index + MINING_REWARD_DELAY);
    validator.db.save_mining_reward(mining_reward).await;
    spawn_update_balances(validator.db.clone(), &block.transactions);
    apply_mining_reward(validator.db.clone(), block.index);
}

pub async fn on_forked_block_received(node: Arc<Mutex<Node>>, mining_flag: Arc<AtomicBool>) {
    println!("Fork detected...");

    let (address, blockchain, max_peer_chain_length) = {
        let locked_node = node.lock().await;
        (locked_node.address.clone(), locked_node.blockchain.clone(), locked_node.max_peer_chain_length.clone())
    };

    if blockchain.invalid_blocks.len() >= 5 && max_peer_chain_length.is_none() {
        println!("5+ forked blocks detected... Resolving fork.");
        mining_flag.store(false, Ordering::Relaxed);

        let message = Message::ChainLengthRequest { from: address.clone() };
        broadcast_message(node.clone(), &message).await;

        tokio::spawn(wait_and_send_block_hashes(node.clone()));
    } else {
        println!("Continuing to mine...");
    }
}

pub async fn on_missing_block_received(node: Arc<Mutex<Node>>, mining_flag: Arc<AtomicBool>, block: &Block) {
    println!("Received block out of order");

    node.lock().await.blockchain.add_orphan_block(block.clone());
    let mut orphan_blocks = node.lock().await.blockchain.orphan_blocks.clone();

    if orphan_blocks.len() >= 5 {
        println!("5+ orphan blocks detected... Resolving missing block.");
        mining_flag.store(false, Ordering::Relaxed);

        let mut combined_chain = node.lock().await.blockchain.chain.clone();
        combined_chain.append(&mut orphan_blocks);
        combined_chain.sort_by_key(|b| b.index);

        let indexes: HashSet<u64> = combined_chain.iter().map(|b| b.index).collect();

        if let (Some(&min), Some(&max)) = (indexes.iter().min(), indexes.iter().max()) {
            let missing_indexes: Vec<u64> = (min..=max)
                .filter(|i| !indexes.contains(i))
                .collect();

            let message = Message::MissingBlocksRequest {
                from: node.lock().await.address.clone(),
                indexes: missing_indexes
            };

            broadcast_message(node.clone(), &message).await;
        }
    } else {
        println!("Continuing to mine...");
    }
}

fn apply_mining_reward(db: DbOperations, block_index: u64) {
    tokio::spawn(async move {
        println!("Applying mining reward for inbound block...");
        let db_response = db.get_mining_reward_at_block_index(block_index).await;
        if db_response.is_err() {
            return;
        }

        let recipient_address = db_response.unwrap().recipient_address;
        db.create_user_and_update_balance(recipient_address, MINING_REWARD_AMOUNT as i64).await;
    });
}