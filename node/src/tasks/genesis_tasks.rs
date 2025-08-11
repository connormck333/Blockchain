use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use crate::chain::block::Block;
use crate::network::message::Message;
use crate::network::message_sender::broadcast_message;
use crate::node::Node;
use crate::tasks::new_node_tasks::request_full_chain;

pub async fn send_genesis_block(node: Arc<Mutex<Node>>, genesis_block: &Block) {
    let genesis_message = Message::GenesisBlock {
        from: node.lock().await.address.clone(),
        genesis_block: genesis_block.clone()
    };

    broadcast_message(node.clone(), &genesis_message).await;
}

pub async fn construct_blockchain(node: Arc<Mutex<Node>>) -> bool {
    println!("Waiting for genesis block...");
    let genesis_block = wait_for_genesis(node.clone()).await;

    if genesis_block.is_none() {
        println!("Failed to receive genesis block. Requesting full chain from peers...");
        let peer_address = node.lock().await.peers.keys().next().unwrap().clone();
        let chain_created = request_full_chain(node.clone(), &peer_address).await;
        if !chain_created {
            println!("Failed to create chain from peers... Exiting.");
            return false;
        } else {
            println!("Chain created successfully from peers.");
        }
    }

    true
}

async fn wait_for_genesis(node: Arc<Mutex<Node>>) -> Option<Block> {
    let mut counter = 0;
    while counter < 10 {
        if node.lock().await.blockchain.get_length() > 0 {
            return Some(node.lock().await.blockchain.chain[0].clone());
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
        counter += 1;
    }

    None
}