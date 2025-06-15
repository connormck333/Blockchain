use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use crate::p2p::client::Client;
use crate::p2p::message::Message;

async fn handle_client(stream: TcpStream, client: Arc<Mutex<Client>>) {
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    loop {
        if let Ok(Some(line)) = lines.next_line().await {
            println!("Received line: {}", line);

            if let Ok(message) = serde_json::from_str::<Message>(&line) {
                match message {
                    Message::PeerConnection { peer_id } => {
                        client.lock().unwrap().add_peer(peer_id.clone());
                        println!("Added peer {}", peer_id);
                        println!("1. Connected peers: {}", client.lock().unwrap().peers.len());
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
    match TcpStream::connect(address).await {
        Ok(stream) => {
            println!("Successfully connected to peer {}", address);

            let (_, mut writer) = stream.into_split();

            let client_address = client.lock().unwrap().address.clone();
            let connection_message = Message::PeerConnection { peer_id: client_address };

            if let Err(e) = writer.write_all(&connection_message.to_vec()).await {
                println!("Failed to write to peer {}: {:?}", address, e);
                return;
            }

            client.lock().unwrap().add_peer(address.to_string());
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

pub async fn start_client(client: Arc<Mutex<Client>>) {
    let client_address = client.lock().unwrap().address.clone();
    let listener = TcpListener::bind(client_address.clone()).await.expect("Failed to bind local port");
    println!("Started client at {}", client_address);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(handle_client(stream, client.clone()));
            }
            Err(e) => {
                println!("Failed to accept connection; err = {:?}", e);
            }
        }
    }
}