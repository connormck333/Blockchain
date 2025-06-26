use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::sync::Mutex;
use crate::block::Block;
use crate::block_validation_type::BlockValidationType;
use crate::constants::{MINING_REWARD_AMOUNT, MINING_REWARD_DELAY};
use crate::database::operations::DbOperations;
use crate::database::validator::Validator;
use crate::mining_reward::MiningReward;
use crate::mining_tasks::spawn_update_balances;
use crate::network::message::{ChainLength, Message};
use crate::network::message_sender::{broadcast_message, send_message};
use crate::network::node::Node;

pub async fn on_genesis_received(node: Arc<Mutex<Node>>, from: String, genesis_block: Block) {
    tokio::time::sleep(Duration::from_millis(1000)).await;
    node.lock().await.blockchain.load_starting_block(genesis_block);
    println!("> Starting block received from {}", from);
    println!("> Starting mining...");
}

pub async fn on_block_received(node: Arc<Mutex<Node>>, mining_flag: Arc<AtomicBool>, validator: Arc<Validator>, from: String, block: Block) {
    for transaction in &block.transactions {
        if !validator.validate_transaction(transaction).await {
            println!("Invalid transaction received... Continuing to mine");
            return;
        }
    }

    let block_validation_type = node.lock().await.receive_block(&block);
    if block_validation_type == BlockValidationType::Valid {
        mining_flag.store(false, Ordering::Relaxed);
        println!("Valid block received from {}... Stopping mining", from);

        let mining_reward = MiningReward::new(MINING_REWARD_AMOUNT, block.miner_address, block.index + MINING_REWARD_DELAY);
        validator.db.save_mining_reward(mining_reward).await;
        spawn_update_balances(validator.db.clone(), block.transactions);
        apply_mining_reward(validator.db.clone(), block.index);
    } else if block_validation_type == BlockValidationType::Fork {
        println!("Fork detected...");

        if node.lock().await.blockchain.invalid_blocks.len() >= 5 {
            println!("5+ forked blocks detected... Resolving fork.");

            let message = Message::ChainLengthRequest { from: node.lock().await.address.clone() };
            broadcast_message(node.clone(), &message).await;

        } else {
            println!("Continuing to mine...");
        }
    } else {
        println!("Invalid block received from {}... Continuing to mine", from);
    }
}

pub async fn on_chain_length_request(node: Arc<Mutex<Node>>, from: String) {
    tokio::spawn(async move {
        let chain_length = node.lock().await.blockchain.get_length();
        let message = Message::ChainLengthResponse { from: node.lock().await.address.clone(), length: chain_length };
        let recipient_node = node.lock().await.get_peer(&from);
        if recipient_node.is_none() {
            println!("No peer found with address: {}", from);
            return;
        }

        send_message(&message, recipient_node.unwrap()).await;
    });
}

pub async fn on_chain_length_response(node: Arc<Mutex<Node>>, message: ChainLength) {
    let mut node = node.lock().await;
    if message.length > node.max_peer_chain_length {
        node.max_peer_chain_length = Some(message.length);
        println!("Updated max_peer_chain_length to {} from peer {}", message.length, message.from);
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