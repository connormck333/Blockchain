use std::collections::HashMap;
use std::str::FromStr;
use futures_lite::StreamExt;
use iroh::{Endpoint, NodeAddr};
use iroh::protocol::Router;
use iroh_gossip::net::{Event, Gossip, GossipEvent, GossipReceiver, GossipSender};
use iroh_gossip::proto::TopicId;
use crate::block::Block;
use crate::network::command::Command;
use crate::network::message::Message;
use crate::network::ticket::Ticket;
use crate::network::args::Args;

pub struct Network {
    nodes: Vec<NodeAddr>,
    topic_id: TopicId,
    args: Args,
    sender: Option<GossipSender>,
    block_tx: Option<tokio::sync::mpsc::Sender<Block>>
}

impl Network {

    pub fn new(args: Args) -> Self {
        let (topic_id, nodes) = Self::get_topic_and_nodes(args.clone());

        Self {
            nodes,
            topic_id,
            args,
            sender: None,
            block_tx: None
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

        let (sender, mut receiver) = gossip.subscribe_and_join(self.topic_id, node_ids).await?.split();
        self.sender = Some(sender);
        println!("> Connected");

        tokio::spawn(async move {
            Self::subscribe_loop(&mut receiver).await
        });

        let (block_tx, mut block_rx) = tokio::sync::mpsc::channel(1);
        self.block_tx = Some(block_tx);

        while let Some(mined_block) = block_rx.recv().await {
            let message = Message::BlockMined {
                from: endpoint.node_id(),
                block: mined_block.clone()
            };

            self.sender.as_mut().unwrap().broadcast(message.to_vec().into()).await?;

            println!("Sent mined block");
        }

        router.shutdown().await?;

        Ok(())
    }

    async fn subscribe_loop(receiver: &mut GossipReceiver) -> anyhow::Result<()> {
        while let Some(event) = receiver.try_next().await? {
            if let Event::Gossip(GossipEvent::Received(msg)) = event {
                match Message::from_bytes(&msg.content)? {
                    Message::BlockMined { from, block } => {
                        println!("> New block with id {} received.", block.index);
                    },
                    _ => {}
                }
            }
        }

        Ok(())
    }

    pub fn broadcast_block_mined(&mut self, block: Block) -> anyhow::Result<()> {
        if self.block_tx.is_some() {
            return Ok(self.block_tx.as_mut().unwrap().blocking_send(block)?)
        }

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