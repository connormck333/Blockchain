use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use crate::args::args::Args;
use crate::args::mode::Mode;
use crate::database::validator::Validator;
use crate::network::message::{ChainLength, Message};
use crate::network::message_receiver::{on_block_received, on_chain_length_request, on_chain_length_response, on_genesis_received};
use crate::network::message_sender::send_message;
use crate::network::node::Node;
use crate::tasks::fork_handling::{get_blocks_with_hash, on_block_hashes_request, on_block_hashes_response};

async fn handle_client(stream: TcpStream, node: Arc<Mutex<Node>>, validator: Arc<Validator>, mining_flag: Arc<AtomicBool>) {
    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();

    loop {
        if let Ok(Some(line)) = lines.next_line().await {
            println!("Received line: {}", line);

            if let Ok(message) = serde_json::from_str::<Message>(&line) {
                match message {
                    Message::PeerConnection { peer_id } => {
                        println!("Received peer connection message from {}", peer_id);
                        let node_clone = node.clone();
                        tokio::spawn(async move {
                            connect_to_peer(node_clone, &peer_id, false).await;
                        });
                    }
                    Message::GenesisBlock {from, genesis_block} => {
                        on_genesis_received(node.clone(), from, genesis_block).await;
                    }
                    Message::BlockMined {from, block} => {
                        on_block_received(node.clone(), mining_flag.clone(), validator.clone(), from, block).await;
                    }
                    Message::ChainLengthRequest { from } => {
                        on_chain_length_request(node.clone(), from).await;
                    }
                    Message::ChainLengthResponse { from, length } => {
                        let chain_length_message = ChainLength {from, length };
                        on_chain_length_response(node.clone(), chain_length_message).await;
                    }
                    Message::BlockHashesRequest { from, hashes } => {
                        on_block_hashes_request(node.clone(), from, hashes).await;
                    }
                    Message::BlockHashesResponse { from, hashes, common_index } => {
                        on_block_hashes_response(node.clone(), mining_flag.clone(), from, hashes, common_index).await;
                    }
                    Message::GetBlocks { from, hashes } => {
                        let blocks_to_send = get_blocks_with_hash(node.clone(), hashes).await;
                        let response = Message::BlockList {
                            from: node.lock().await.address.clone(),
                            blocks: blocks_to_send
                        };

                        let mut response_bytes = response.to_vec();
                        response_bytes.push(b'\n');

                        if let Err(e) = writer.write_all(&response_bytes).await {
                            println!("Failed to send blocks: {:?}", e);
                        } else {
                            println!("Sent blocks to {}", from.clone());
                        }
                    }
                    _ => {
                        println!("Received unknown message");
                    }
                }
            } else {
                println!("Failed to deserialize line.");
            }
        }
    }
}

pub async fn connect_to_peer(node: Arc<Mutex<Node>>, peer_address: &str, return_address: bool) {
    println!("Connecting to {}", peer_address);
    match TcpStream::connect(peer_address).await {
        Ok(stream) => {
            println!("Successfully connected to peer {}", peer_address);

            let (reader, mut writer) = stream.into_split();

            if return_address {
                let client_address = node.lock().await.address.clone();
                let connection_message = Message::PeerConnection { peer_id: client_address.clone() };

                send_message(&connection_message, &mut writer).await;
            }

            node.lock().await.add_peer(peer_address.to_string(), writer, reader);
        }
        Err(e) => {
            println!("Failed to connect to peer {}: {:?}", peer_address, e);
            println!("Retrying in 5 seconds...");
            tokio::time::sleep(Duration::from_secs(5)).await;
            println!("Retrying now...");
            let _ = connect_to_peer(node, peer_address, return_address);
        }
    }
}

pub async fn start_client(node: Arc<Mutex<Node>>, address: String, validator: Arc<Validator>, mining_flag: Arc<AtomicBool>) {
    let listener = TcpListener::bind(address.clone()).await.expect("Failed to bind local port");
    println!("Started client at {}", address);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                println!("Accepted connection from {}", stream.peer_addr().unwrap());
                tokio::spawn(handle_client(stream, node.clone(), validator.clone(), mining_flag.clone()));
            }
            Err(e) => {
                println!("Failed to accept connection; err = {:?}", e);
            }
        }
    }
}

pub async fn create_node(args: &Args, validator: Arc<Validator>, mining_flag: Arc<AtomicBool>) -> Arc<Mutex<Node>> {
    let (node_address, peer_address) = match args.node_type.get_mode() {
        Mode::OPEN { node_address } => (node_address.clone(), None),
        Mode::JOIN { node_address, peer_address } => (node_address.clone(), Some(peer_address.clone()))
    };

    let node = Arc::new(Mutex::new(Node::new(node_address.clone())));
    start_peer_connection(node.clone(), validator, mining_flag, peer_address).await;

    node
}

pub async fn start_peer_connection(
    node: Arc<Mutex<Node>>,
    validator: Arc<Validator>,
    mining_flag: Arc<AtomicBool>,
    peer_address: Option<String>
) {
    let node_address = node.lock().await.address.clone();
    tokio::spawn(start_client(node.clone(), node_address, validator, mining_flag));

    if peer_address.is_some() {
        let node_copy = node.clone();
        let peer_address_clone = peer_address.unwrap();
        tokio::spawn(async move {
            connect_to_peer(node_copy, &peer_address_clone, true).await;
        });
    }
}