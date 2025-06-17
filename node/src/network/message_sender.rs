use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use crate::block::Block;
use crate::network::message::Message;
use crate::network::node::Node;

pub struct MessageSender {
    node: Arc<Mutex<Node>>
}

impl MessageSender {
    pub fn new(node: Arc<Mutex<Node>>) -> Self {
        Self {
            node
        }
    }

    pub async fn send_genesis_block(&mut self, genesis_block: &Block) {
        let mut node = self.node.lock().await;
        let genesis_message = serde_json::to_string(&Message::GenesisBlock {
            from: node.address.clone(),
            genesis_block: genesis_block.clone()
        }).expect("Failed to serialize genesis block message");

        for (peer_address, writer) in &mut node.peers {
            if let Err(e) = writer.write_all(genesis_message.as_bytes()).await {
                println!("Failed to send genesis block to peer {}: {:?}", peer_address, e);
            }
        }
    }
}