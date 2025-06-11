use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use crate::p2p::message::Message;

async fn handle_client(stream: TcpStream) {
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        println!("Received line: {}", line);

        if let Ok(message) = serde_json::from_str::<Message>(&line) {
            match message {
                Message::PING { timestamp } => {
                    println!("PING: {}", timestamp);

                    let pong = Message::create_pong();
                    let pong_json = format!("{}\n", pong);
                    if let Err(e) = lines.get_mut().write_all(pong_json.as_bytes()).await {
                        println!("Failed to send pong: {}", e);
                        break;
                    } else {
                        println!("Successfully sent pong");
                    }
                }
                Message::PONG { timestamp } => {
                    println!("PONG: {}", timestamp);
                }
            }
        } else {
            println!("Failed to deserialize line.");
        }
    }

    println!("Connection closed.");
}

pub async fn connect_to_peer(address: &str) {
    match TcpStream::connect(address).await {
        Ok(stream) => {
            println!("Successfully connected to peer {}", address);

            let (reader, mut writer) = stream.into_split();
            let mut lines = BufReader::new(reader).lines();

            let ping = Message::create_ping();
            let ping_msg = format!("{}\n", ping);

            // Send the PING message with newline delimiter
            if let Err(e) = writer.write_all(ping_msg.as_bytes()).await {
                println!("Failed to write to peer {}: {:?}", address, e);
                return;
            }
            println!("Successfully sent PING to peer {}", address);

            // Wait for PONG response
            while let Ok(Some(line)) = lines.next_line().await {
                if let Ok(message) = serde_json::from_str::<Message>(&line) {
                    match message {
                        Message::PONG { timestamp } => {
                            println!("Received PONG from {}: {}", address, timestamp);
                        }
                        other => {
                            println!("Received unexpected message from {}: {:?}", address, other);
                        }
                    }
                } else {
                    println!("Failed to deserialize message from {}", address);
                }
            }
            println!("Connection to {} closed.", address);
        }
        Err(e) => {
            println!("Failed to connect to peer {}: {:?}", address, e);
            println!("Retrying in 5 seconds...");
            tokio::time::sleep(Duration::from_secs(5)).await;
            let _ = connect_to_peer(address);
        }
    }
}

pub async fn start_connection(address: &str) {
    let listener = TcpListener::bind(address).await.expect("Failed to bind local port");
    println!("Listening on port 8080");

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(handle_client(stream));
            }
            Err(e) => {
                println!("Failed to accept connection; err = {:?}", e);
            }
        }
    }
}