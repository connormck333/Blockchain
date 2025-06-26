use std::sync::Arc;
use tokio::sync::Mutex;
use crate::network::message::Message;
use crate::network::message_sender::send_message;
use crate::network::node::Node;

pub async fn wait_for_length_responses(node: Arc<Mutex<Node>>) {
    // Sleep for 10 seconds to allow time for length responses to be received
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    tokio::spawn(async move {
        let mut locked_node = node.lock().await;
        if let Some(max_length) = locked_node.max_peer_chain_length {
            let latest_block_index = locked_node.blockchain.get_latest_block().index as usize;
            let message = Message::BlockAtIndexRequest { from: locked_node.address.clone(), index: latest_block_index + 1 };
            let recipient_node = locked_node.get_peer(max_length.from);

            send_message(&message, recipient_node).await;
        } else {
            println!("No peer chain lengths received.");
        }
    });
}