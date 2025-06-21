use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::Mutex;
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

    pub async fn broadcast_message(&mut self, message: &Message) {
        let mut node = self.node.lock().await;

        for (_, mut writer) in &mut node.peers {
            Self::send_message(message, &mut writer).await;
        }
    }

    pub async fn send_message(message: &Message, writer: &mut OwnedWriteHalf) {
        println!("Sending message to peer");
        let mut msg_bytes = message.to_vec();
        msg_bytes.push(b'\n');
        if let Err(e) = writer.write_all(&msg_bytes).await {
            println!("Failed to write to peer: {:?}", e);
            return;
        }
    }
}