use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;
use tokio::sync::Mutex;
use crate::block::Block;
use crate::block_validation_type::BlockValidationType;
use crate::database::validator::Validator;
use crate::network::message::{ChainLength, Message};
use crate::network::message_sender::send_message;
use crate::network::node::Node;
use crate::tasks::block_validation::{on_forked_block_received, on_missing_block_received, on_valid_block_received};

pub async fn on_genesis_received(node: Arc<Mutex<Node>>, from: String, genesis_block: Block) {
    tokio::time::sleep(Duration::from_millis(1000)).await;
    node.lock().await.blockchain.load_starting_block(genesis_block);
    println!("> Starting block received from {}", from);
    println!("> Starting mining...");
}

pub async fn on_block_received(node: Arc<Mutex<Node>>, mining_flag: Arc<AtomicBool>, validator: Arc<Validator>, from: String, block: Block) {
    if node.lock().await.blockchain_locked {
        // Save block & exit if blockchain is not ready
        println!("Block received while blockchain is locked... Adding to pending blocks");
        node.lock().await.blockchain.add_pending_block(block.clone());
        return;
    }

    for transaction in &block.transactions {
        if !validator.validate_transaction(transaction).await {
            println!("Invalid transaction received... Continuing to mine");
            return;
        }
    }

    let block_validation_type = node.lock().await.receive_block(&block);
    match block_validation_type {
        BlockValidationType::Valid => {
            on_valid_block_received(validator.clone(), mining_flag.clone(), &block, &from).await;
        },
        BlockValidationType::Fork => {
            on_forked_block_received(node.clone(), mining_flag.clone()).await;
        },
        BlockValidationType::Missing => {
            on_missing_block_received(node.clone(), mining_flag.clone(), &block).await;
        }
        BlockValidationType::Invalid => {
            println!("Invalid block received from {}... Continuing to mine", from);
        }
    }
}

pub async fn on_chain_length_request(node: Arc<Mutex<Node>>, from: String) {
    tokio::spawn(async move {
        let mut locked_node = node.lock().await;
        let chain_length = locked_node.blockchain.get_length();
        let message = Message::ChainLengthResponse { from: locked_node.address.clone(), length: chain_length };
        let recipient_node = locked_node.get_peer(&from);

        if let Some(peer) = recipient_node {
            send_message(&message, &mut peer.writer).await;
        } else {
            println!("No peer found with address: {}", from);
        }
    });
}

pub async fn on_chain_length_response(node: Arc<Mutex<Node>>, message: ChainLength) {
    let current_max_length = node.lock().await.max_peer_chain_length.clone();

    if current_max_length.is_none() || message.length > current_max_length.unwrap().length {
        node.lock().await.max_peer_chain_length = Some(ChainLength {
            from: message.from.clone(),
            length: message.length
        });
        println!("Updated max_peer_chain_length to {} from peer {}", message.length, message.from);
        return;
    }
}