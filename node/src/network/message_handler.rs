use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use iroh::NodeId;
use iroh_gossip::net::Message as IrohMessage;
use crate::network::message::Message;
use tokio::sync::Mutex;
use crate::block::Block;
use crate::constants::{MINING_REWARD_AMOUNT, MINING_REWARD_DELAY};
use crate::database::connection::Connection;
use crate::database::validator::Validator;
use crate::mining_reward::MiningReward;
use crate::network::node::Node;

pub async fn handle_incoming_message(node: Arc<Mutex<Node>>, mining_flag: Arc<AtomicBool>, validator: Arc<Validator>, msg: IrohMessage) {
    match Message::from_bytes(&msg.content) {
        Ok(parsed) => {
            match parsed {
                Message::GenesisBlock {from, genesis_block} => {
                    on_genesis_received(node.clone(), from, genesis_block).await;
                }
                Message::BlockMined {from, block} => {
                    on_block_received(node, mining_flag, validator, from, block).await;
                }
                _ => {
                    println!("Unknown message received");
                }
            }
        },
        Err(e) => eprintln!("Failed to parse message: {e}"),
    }
}

async fn on_genesis_received(node: Arc<Mutex<Node>>, from: NodeId, genesis_block: Block) {
    tokio::time::sleep(Duration::from_millis(1000)).await;
    node.lock().await.blockchain.load_starting_block(genesis_block);
    println!("> Starting block received from {}", from);
    println!("> Starting mining...");
}

async fn on_block_received(node: Arc<Mutex<Node>>, mining_flag: Arc<AtomicBool>, validator: Arc<Validator>, from: NodeId, block: Block) {
    for transaction in &block.transactions {
        if !validator.validate_transaction(transaction).await {
            println!("Invalid transaction received... Continuing to mine");
            return;
        }
    }

    if node.lock().await.receive_block(&block) {
        mining_flag.store(false, Ordering::Relaxed);
        println!("Valid block received from {}... Stopping mining", from);

        let mining_reward = MiningReward::new(MINING_REWARD_AMOUNT, block.miner_address, block.index + MINING_REWARD_DELAY);
        validator.db_connection.save_mining_reward(mining_reward).await;
        apply_mining_reward(validator.db_connection.clone(), block.index);
    } else {
        println!("Invalid block received from {}... Continuing to mine", from);
    }
}

fn apply_mining_reward(db_connection: Arc<Connection>, block_index: u64) {
    tokio::spawn(async move {
        println!("Applying mining reward for inbound block...");
        let db_response = db_connection.get_mining_reward_at_block_index(block_index).await;
        if db_response.is_err() {
            return;
        }

        let recipient_address = db_response.unwrap().recipient_address;
        let user_exists = db_connection.create_user_if_not_exists(&recipient_address, MINING_REWARD_AMOUNT).await;
        if user_exists {
            // Increment user balance if already exists in db
            // Otherwise, the balance will be saved on user creation
            db_connection.increment_user_balance(recipient_address, MINING_REWARD_AMOUNT).await;
        }
    });
}