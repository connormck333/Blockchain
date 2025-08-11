use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::Mutex;
use crate::network::message::Message;
use crate::node::Node;

pub async fn broadcast_message(node: Arc<Mutex<Node>>, message: &Message) {
    let mut node = node.lock().await;

    for (_, peer) in &mut node.peers {
        send_message(message, &mut peer.writer).await;
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

pub async fn send_message_expect_response(message: &Message, writer: &mut OwnedWriteHalf, reader: &mut OwnedReadHalf) -> Option<Message> {
    let mut msg_bytes = message.to_vec();
    msg_bytes.push(b'\n');
    if let Err(e) = writer.write_all(&msg_bytes).await {
        println!("Failed to write to peer: {:?}", e);
        return None;
    } else if writer.flush().await.is_err() {
        return None;
    }

    let mut buf_reader = BufReader::new(reader);
    let mut response = String::new();
    match buf_reader.read_line(&mut response).await {
        Ok(0) => None,
        Ok(_) => {
            if let Ok(message) = Message::from_bytes(response.as_bytes()) {
                Some(message)
            } else {
                println!("Failed to parse response message");
                None
            }
        },
        Err(_) => None,
    }
}