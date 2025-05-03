use std::collections::HashMap;
use std::str::FromStr;
use futures_lite::StreamExt;
use iroh::{Endpoint, NodeAddr};
use iroh::protocol::Router;
use iroh_gossip::net::{Event, Gossip, GossipEvent, GossipReceiver};
use iroh_gossip::proto::TopicId;
use crate::network::command::Command;
use crate::network::message::Message;
use crate::network::ticket::Ticket;
use crate::network::args::Args;

pub struct Network {
    nodes: Vec<NodeAddr>,
    topic_id: TopicId,
    args: Args,
}

impl Network {

    pub fn new(args: Args) -> Self {
        let (topic_id, nodes) = Self::get_topic_and_nodes(args.clone());

        Self {
            nodes,
            topic_id,
            args
        }
    }

    pub async fn connect(&mut self) -> anyhow::Result<()> {
        let endpoint = Endpoint::builder().discovery_n0().bind().await?;

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
            Ticket { topic: self.topic_id, peers: nodes }
        };
        println!("> Ticket to join us: {ticket}");

        let node_ids = self.nodes.clone().iter().map(|p| p.node_id).collect();
        if self.nodes.is_empty() {
            println!("> Waiting for nodes to join");
        } else {
            println!("> Joining nodes: {}", self.nodes.len());
            for node in self.nodes.clone().into_iter() {
                endpoint.add_node_addr(node)?
            }
        }

        let (sender, receiver) = gossip.subscribe_and_join(self.topic_id, node_ids).await?.split();
        println!("> Connected");

        if let Some(name) = self.args.clone().name {
            let message = Message::AboutMe {
                from: endpoint.node_id(),
                name
            };

            sender.broadcast(message.to_vec().into()).await?;
        }

        tokio::spawn(Self::subscribe_loop(receiver));

        let (line_tx, mut line_rx) = tokio::sync::mpsc::channel(1);
        std::thread::spawn(move || Self::input_loop(line_tx));
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

    fn get_topic_and_nodes(args: Args) -> (TopicId, Vec<NodeAddr>) {
        match &args.command {
            Command::OPEN => {
                let topic = TopicId::from_bytes(rand::random());
                println!("> Opening chatroom for topic {topic}");
                (topic, vec![])
            }
            Command::JOIN { ticket } => {
                let ticket = Ticket::from_str(ticket).unwrap();
                println!("> Joining chat room for topic {}", ticket.topic);
                (ticket.topic, ticket.peers)
            }
        }
    }

    async fn subscribe_loop(mut receiver: GossipReceiver) -> anyhow::Result<()> {
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

    fn input_loop(line_tx: tokio::sync::mpsc::Sender<String>) -> anyhow::Result<()> {
        let mut buffer = String::new();
        let stdin = std::io::stdin();
        loop {
            stdin.read_line(&mut buffer)?;
            line_tx.blocking_send(buffer.clone())?;
            buffer.clear();
        }
    }
}