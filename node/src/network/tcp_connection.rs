use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use crate::args::args::Args;
use crate::args::mode::Mode;
use crate::network::client::Client;
use crate::network::message::Message;

async fn handle_client(stream: TcpStream, client: Arc<Mutex<Client>>) {
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    loop {
        if let Ok(Some(line)) = lines.next_line().await {
            println!("Received line: {}", line);

            if let Ok(message) = serde_json::from_str::<Message>(&line) {
                match message {
                    Message::PeerConnection { peer_id } => {
                        client.lock().await.add_peer(peer_id.clone());
                        println!("Added peer {}", peer_id);
                        println!("1. Connected peers: {}", client.lock().await.peers.len());
                    }
                    Message::GenesisBlock {from, genesis_block} => {
                        // on_genesis_received(node.clone(), from, genesis_block).await;
                    }
                    Message::BlockMined {from, block} => {
                        // on_block_received(node, mining_flag, validator, from, block).await;
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

pub async fn connect_to_peer(client: Arc<Mutex<Client>>, address: &str) {
    println!("Connecting to {}", address);
    match TcpStream::connect(address).await {
        Ok(stream) => {
            println!("Successfully connected to peer {}", address);

            let (_, mut writer) = stream.into_split();

            let client_address = client.lock().await.address.clone();
            let connection_message = Message::PeerConnection { peer_id: client_address };

            if let Err(e) = writer.write_all(&connection_message.to_vec()).await {
                println!("Failed to write to peer {}: {:?}", address, e);
                return;
            }

            client.lock().await.add_peer(address.to_string());
        }
        Err(e) => {
            println!("Failed to connect to peer {}: {:?}", address, e);
            println!("Retrying in 5 seconds...");
            tokio::time::sleep(Duration::from_secs(5)).await;
            println!("Retrying now...");
            let _ = connect_to_peer(client, address);
        }
    }
}

pub async fn start_client(client: Arc<Mutex<Client>>, client_address: String) {
    let listener = TcpListener::bind(client_address.clone()).await.expect("Failed to bind local port");
    println!("Started client at {}", client_address);

    match listener.accept().await {
        Ok((stream, _)) => {
            tokio::spawn(handle_client(stream, client.clone()));
        }
        Err(e) => {
            println!("Failed to accept connection; err = {:?}", e);
        }
    }

    println!("End of start client");
}

pub fn create_client(args: &Args) -> Arc<Mutex<Client>> {
    let (node_address, peer_address) = match args.node_type.get_mode() {
        Mode::OPEN { node_address } => (node_address.clone(), "".to_string()),
        Mode::JOIN { node_address, peer_address } => (node_address.clone(), peer_address.clone())
    };

    let mut client = Arc::new(Mutex::new(Client::new(node_address.clone())));

    tokio::spawn(start_client(client.clone(), node_address));

    if peer_address.len() > 0 {
        let client_copy = client.clone();
        tokio::spawn(async move {
            let peer_address = peer_address.clone();
            connect_to_peer(client_copy, peer_address.as_str()).await;
        });
    }

    client
}