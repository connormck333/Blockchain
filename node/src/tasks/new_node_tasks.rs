use std::sync::Arc;
use tokio::sync::Mutex;
use crate::chain::block::Block;
use crate::network::message::Message;
use crate::network::message_sender::send_message_expect_response;
use crate::node::Node;

pub async fn create_full_chain_response(node: Arc<Mutex<Node>>) -> Message {
    let chain = node.lock().await.blockchain.chain.clone();
    let response = Message::FullChainResponse {
        from: node.lock().await.address.clone(),
        blocks: chain
    };

    response
}

pub async fn request_full_chain(node: Arc<Mutex<Node>>, peer_address: &String) -> bool {
    let mut locked_node = node.lock().await;
    let request = Message::FullChainRequest {
        from: locked_node.address.clone()
    };

    let recipient = locked_node.get_peer(peer_address);
    if let Some(peer) = recipient {
        let response = send_message_expect_response(&request, &mut peer.writer, &mut peer.reader).await;

        if let Some(message) = response {
            match message {
                Message::FullChainResponse { from, blocks } => {
                    println!("Received full chain from {} with {} blocks", from, blocks.len());
                    locked_node.blockchain.chain = merge_pending_and_received_blocks(
                        &blocks,
                        locked_node.blockchain.pending_blocks.clone()
                    ).await;

                    return true;
                },
                _ => {
                    println!("Unexpected response type from peer {}", peer_address);
                }
            }
        } else {
            println!("No response received from peer {}", peer_address);
        }
    }

    false
}

async fn merge_pending_and_received_blocks(received_blocks: &Vec<Block>, mut pending_blocks: Vec<Block>) -> Vec<Block> {
    pending_blocks.sort_by_key(|block| block.index);

    let mut recent_index = received_blocks.last().unwrap().index as usize;
    let mut merged_blocks: Vec<Block> = received_blocks.clone();

    for block in pending_blocks {
        if block.index as usize == recent_index + 1 {
            merged_blocks.push(block);
            recent_index += 1;
        }
    }

    merged_blocks
}