use std::collections::HashMap;
use std::str::FromStr;
use clap::Parser;
use futures_lite::StreamExt;
use iroh::Endpoint;
use iroh::protocol::Router;
use iroh_gossip::net::{Event, Gossip, GossipEvent, GossipReceiver};
use iroh_gossip::proto::TopicId;
use anyhow::Result;
use crate::network::args::Args;
use crate::network::command::Command;
use crate::network::message::Message;
use crate::network::ticket::Ticket;

mod block;
mod transaction;
mod blockchain;
mod utils;
mod wallet;
mod network;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let (topic, nodes) = match &args.command {
        Command::OPEN => {
            let topic = TopicId::from_bytes(rand::random());
            println!("> Opening chatroom for topic {topic}");
            (topic, vec![])
        }
        Command::JOIN { ticket } => {
            let Ticket { topic, peers: nodes } = Ticket::from_str(ticket)?;
            println!("> Joining chat room for topic {topic}");
            (topic, nodes)
        }
    };

    let endpoint: Endpoint = Endpoint::builder()
        .discovery_n0()
        .bind()
        .await?;

    let gossip: Gossip = Gossip::builder()
        .spawn(endpoint.clone())
        .await?;

    let router: Router = Router::builder(endpoint.clone())
        .accept(iroh_gossip::ALPN, gossip.clone())
        .spawn()
        .await?;

    let ticket = {
        let me = endpoint.node_addr().await?;
        let nodes = vec![me];
        Ticket { topic, peers: nodes }
    };
    println!("> Ticket to join us: {ticket}");

    let node_ids = nodes.iter().map(|p| p.node_id).collect();
    if nodes.is_empty() {
        println!("> Waiting for nodes to join");
    } else {
        println!("> Joining nodes: {nodes:?}");
        for node in nodes.into_iter() {
            endpoint.add_node_addr(node)?
        }
    }

    let (sender, receiver) = gossip.subscribe_and_join(topic, node_ids).await?.split();
    println!("> Connected");

    if let Some(name) = args.name {
        let message = Message::AboutMe {
            from: endpoint.node_id(),
            name
        };

        sender.broadcast(message.to_vec().into()).await?;
    }

    tokio::spawn(subscribe_loop(receiver));

    let (line_tx, mut line_rx) = tokio::sync::mpsc::channel(1);
    std::thread::spawn(move || input_loop(line_tx));
    println!("> type a message and hit enter to broadcast...");

    while let Some(text) = line_rx.recv().await {
        let message = Message::Message {
            from: endpoint.node_id(),
            text: text.clone()
        };

        sender.broadcast(message.to_vec().into()).await?;

        println!("Sent: {text}");
    }

    router.shutdown().await?;

    Ok(())
}

async fn subscribe_loop(mut receiver: GossipReceiver) -> Result<()> {
    let mut names = HashMap::new();

    while let Some(event) = receiver.try_next().await? {
        if let Event::Gossip(GossipEvent::Received(msg)) = event {
            match Message::from_bytes(&msg.content)? {
                Message::AboutMe { from, name } => {
                    names.insert(from, name.clone());
                    println!("> {} is now known as {}", from.fmt_short(), name);
                }
                Message::Message { from, text } => {
                    let name = names
                        .get(&from)
                        .map_or_else(|| from.fmt_short(), String::to_string);
                    println!("> {}: {}", name, text);
                }
            }
        }
    }

    Ok(())
}

fn input_loop(line_tx: tokio::sync::mpsc::Sender<String>) -> Result<()> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    loop {
        stdin.read_line(&mut buffer)?;
        line_tx.blocking_send(buffer.clone())?;
        buffer.clear();
    }
}