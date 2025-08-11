use std::sync::{atomic, Arc};
use std::sync::atomic::AtomicBool;
use tokio::sync::Mutex;
use crate::chain::block::Block;
use crate::network::message::Message;
use crate::network::message_sender::{send_message, send_message_expect_response};
use crate::node::Node;

pub async fn wait_and_send_block_hashes(node: Arc<Mutex<Node>>) {
    // Sleep for 10 seconds to allow time for length responses to be received
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    let (max_peer_chain_length, blockchain, address) = {
        let locked_node = node.lock().await;
        (
            locked_node.max_peer_chain_length.clone(),
            locked_node.blockchain.clone(),
            locked_node.address.clone(),
        )
    };

    tokio::spawn(async move {
        if let Some(max_length) = max_peer_chain_length {
            let hashes = blockchain.chain
                .iter()
                .rev()
                .map(|block| block.hash.clone())
                .collect();

            let message = Message::BlockHashesRequest {
                from: address,
                hashes
            };
            if let Some(recipient_node) = node.lock().await.get_peer(max_length.from.clone().as_str()) {
                send_message(&message, &mut recipient_node.writer).await;
            } else {
                println!("No peer found to send block hashes request.");
            }
        } else {
            println!("No peer chain lengths received.");
        }
    });
}

pub async fn on_block_hashes_request(node: Arc<Mutex<Node>>, from: String, hashes: Vec<String>) {
    let (blockchain, node_address) = {
        let locked_node = node.lock().await;
        (locked_node.blockchain.clone(), locked_node.address.clone())
    };

    for hash in hashes {
        if let Some(overlap_block) = blockchain.chain.iter().find(|b| b.hash == hash) {
            println!("Overlap found with block {} from {}", overlap_block.index, from);

            let overlap_index = overlap_block.index as usize;
            let response_hashes = blockchain.chain[overlap_index..]
                .iter()
                .map(|b| b.hash.clone())
                .collect();

            let message = Message::BlockHashesResponse {
                from: node_address.clone(),
                hashes: response_hashes,
                common_index: overlap_index
            };

            if let Some(peer) = node.lock().await.get_peer(&from) {
                send_message(&message, &mut peer.writer).await;
                return;
            }
        }
    }

    println!("No overlap found with received hashes from {}", from);
}

pub async fn on_block_hashes_response(node: Arc<Mutex<Node>>, mining_flag: Arc<AtomicBool>, from: String, hashes: Vec<String>, common_index: usize) {
    let max_peer_chain_length = node.lock().await.max_peer_chain_length.clone();
    if let Some(expected_peer) = max_peer_chain_length {
        if expected_peer.from != from {
            println!("Ignoring block hashes response from unexpected peer: {}", from);
            return;
        }

        let mut blockchain = node.lock().await.blockchain.chain.clone()[..common_index + 1].to_vec();
        let invalid_blocks = node.lock().await.blockchain.invalid_blocks.clone();
        let mut missing_blocks: Vec<String> = vec![];
        let mut valid_blocks: Vec<Block> = vec![];

        for hash in hashes {
            if let Some(block) = invalid_blocks.iter().find(|b| b.hash == hash) {
                valid_blocks.push(block.clone());
            } else {
                missing_blocks.push(hash);
            }
        }

        let blocks_response = send_get_blocks_request(node.clone(), missing_blocks, &from).await;
        if let Some(msg) = blocks_response {
            if let Message::BlockList { blocks, .. } = msg {
                valid_blocks.extend(blocks);
                valid_blocks.sort_by_key(|block| block.index);

                let mut last_index = common_index as u64;
                for block in valid_blocks {
                    if block.index == last_index + 1 {
                        blockchain.push(block);
                        last_index += 1;
                    } else {
                        println!("Skipping block {} as it is not the next in sequence after {}", block.index, last_index);
                    }
                }

                node.lock().await.blockchain.chain = blockchain;
                node.lock().await.blockchain.invalid_blocks = vec![];
                println!("Received and added blocks from peer {}", from);

                mining_flag.store(true, atomic::Ordering::Release);
            } else {
                println!("Unexpected message type received in block hashes response.");
            }
        } else {
            println!("Failed to retrieve missing blocks from peer {}", from);
        }
    }
}

pub async fn send_get_blocks_request(node: Arc<Mutex<Node>>, hashes: Vec<String>, recipient: &String) -> Option<Message> {
    let node_address = node.lock().await.address.clone();

    let message = Message::GetBlocks {
        from: node_address,
        hashes
    };

    if let Some(peer) = node.lock().await.get_peer(recipient) {
        return send_message_expect_response(&message, &mut peer.writer, &mut peer.reader).await;
    } else {
        println!("No peer found to send get blocks request.");
    }

    None
}

pub async fn get_blocks_with_hash(node: Arc<Mutex<Node>>, hashes: Vec<String>) -> Vec<Block> {
    let blockchain = node.lock().await.blockchain.clone();
    let mut blocks_to_send: Vec<Block> = vec![];

    for hash in hashes {
        if let Some(block) = blockchain.chain.iter().find(|b| b.hash == hash) {
            blocks_to_send.push(block.clone());
        }
    }

    blocks_to_send
}