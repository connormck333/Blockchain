use std::sync::Arc;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use crate::network::message::Message;
use crate::network::message_sender::send_message;
use crate::node::Node;

pub fn spawn_connect_to_many_peers(node: Arc<Mutex<Node>>, peer_addresses: Vec<String>) {
    tokio::spawn(async move {
        for peer_address in peer_addresses {
            if should_connect_to_peer(node.clone(), &peer_address).await {
                println!("Connecting to unknown peer: {}", peer_address);
                connect_to_peer(node.clone(), &peer_address, true).await;
            } else {
                println!("Already connected to peer: {}", peer_address);
            }
        }
    });
}

pub fn spawn_peer_connection_task(node: Arc<Mutex<Node>>, peer_address: &str) {
    let address_clone = peer_address.to_string();
    tokio::spawn(async move {
        connect_to_peer(node.clone(), &address_clone, false).await;
    });
}

pub fn spawn_initial_peer_connection(node: Arc<Mutex<Node>>, peer_address: &str) {
    let address_clone = peer_address.to_string();
    tokio::spawn(async move {
        connect_to_peer(node.clone(), &address_clone, true).await;
    });
}

async fn connect_to_peer(node: Arc<Mutex<Node>>, peer_address: &str, initial_connection: bool) {
    println!("Received peer connection message from {}", peer_address);

    if !should_connect_to_peer(node.clone(), peer_address).await {
        println!("Already connected to peer: {}", peer_address);
        return;
    }

    match TcpStream::connect(peer_address).await {
        Ok(stream) => {
            println!("Successfully connected to peer {}", peer_address);
            let (reader, mut writer) = stream.into_split();

            if initial_connection {
                send_peer_connection_request(node.clone(), &mut writer).await;
            } else {
                send_known_addresses(node.clone(), &mut writer).await;
            }

            // Save connection
            node.lock().await.add_peer(peer_address.to_string(), writer, reader);
        }
        Err(e) => {
            println!("Failed to connect to peer {}: {:?}", peer_address, e);
        }
    }
}

async fn send_known_addresses(node: Arc<Mutex<Node>>, recipient: &mut OwnedWriteHalf) {
    let peers: Vec<String> = node.lock().await.peers.iter()
        .map(|p| p.1.address.clone())
        .collect();

    let message = Message::PeerConnectionResponse {
        from: node.lock().await.address.clone(),
        known_addresses: peers
    };

    send_message(&message, recipient).await;
}

async fn send_peer_connection_request(node: Arc<Mutex<Node>>, recipient: &mut OwnedWriteHalf) {
    let node_address = node.lock().await.address.clone();
    let message = Message::PeerConnectionRequest {
        from: node_address
    };

    send_message(&message, recipient).await;
}

async fn should_connect_to_peer(node: Arc<Mutex<Node>>, peer_address: &str) -> bool {
    let locked_node = node.lock().await;
    !locked_node.peers.contains_key(peer_address) && !locked_node.address.eq(peer_address)
}