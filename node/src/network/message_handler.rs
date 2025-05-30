use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use iroh::NodeId;
use iroh_gossip::net::Message as IrohMessage;
use crate::network::message::Message;
use tokio::sync::Mutex;
use crate::block::Block;
use crate::database::validator::Validator;
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

    if node.lock().await.receive_block(block) {
        mining_flag.store(false, Ordering::Relaxed);
        println!("Valid block received from {}... Stopping mining", from);
    } else {
        println!("Invalid block received from {}... Continuing to mine", from);
    }
}