use std::str::FromStr;
use std::sync::{Arc};
use tokio::sync::Mutex;
use futures_lite::StreamExt;
use iroh::{Endpoint, NodeAddr};
use iroh::protocol::Router;
use iroh_gossip::net::{Event, Gossip, GossipEvent, GossipReceiver};
use iroh_gossip::proto::TopicId;
use crate::block::Block;
use crate::network::command::Command;
use crate::network::message::Message;
use crate::network::ticket::Ticket;
use crate::network::args::Args;
use crate::network::node::Node;

pub struct Network {
    connected_nodes: Vec<NodeAddr>,
    topic_id: TopicId,
    args: Args,
    gossip: Option<Gossip>
}

impl Network {

    pub fn new(args: Args) -> Self {
        let (topic_id, nodes) = Self::get_topic_and_nodes(args.clone());

        Self {
            connected_nodes: nodes,
            topic_id,
            args,
            gossip: None
        }
    }

    pub async fn connect(&mut self, node: Arc<Mutex<Node>>) -> anyhow::Result<()> {
        let endpoint = Endpoint::builder().discovery_n0().bind().await?;
        println!("node id: {}", endpoint.node_id());

        self.gossip = Some(Gossip::builder()
            .spawn(endpoint.clone())
            .await?);

        let router: Router = Router::builder(endpoint.clone())
            .accept(iroh_gossip::ALPN, self.gossip.as_ref().unwrap().clone())
            .spawn()
            .await?;

        let ticket = {
            let me = endpoint.node_addr().await?;
            let nodes = vec![me];
            Ticket { topic: self.topic_id, peers: nodes }
        };
        println!("> Ticket to join us: {ticket}");

        let node_ids = self.connected_nodes.clone().iter().map(|p| p.node_id).collect();
        if self.connected_nodes.is_empty() {
            println!("> Waiting for nodes to join");
        } else {
            println!("> Joining nodes: {}", self.connected_nodes.len());
            for node in self.connected_nodes.clone().into_iter() {
                println!("{}", node.clone().node_id);
                endpoint.add_node_addr(node)?
            }
        }

        let (sender, receiver) = self.gossip.as_mut().unwrap().subscribe_and_join(self.topic_id, node_ids).await?.split();
        println!("> Connected");

        // let (block_tx, mut block_rx) = tokio::sync::mpsc::channel(1);

        {
            let node_clone = node.clone();
            tokio::spawn(async move {
                let mined_block: Option<Block> = tokio::task::spawn_blocking(move || {
                    node_clone.blocking_lock().mine_block()
                }).await.unwrap();

                if let Some(block) = mined_block {
                    let message = Message::BlockMined {
                        from: endpoint.node_id(),
                        block
                    };
                    let bytes = message.to_vec().into();
                    println!("Sending mined block");
                    // let _ = block_tx.send(bytes).await;
                    let _ = sender.broadcast(bytes).await;
                    println!("Sent mined block");
                }
            });
        }

        // Listens for incoming messages
        tokio::spawn(Self::subscribe_loop(receiver));

        // {
        //     tokio::spawn(async move {
        //         while let Some(msg) = block_rx.recv().await {
        //             match Message::from_bytes(&msg) {
        //                 Ok(Message::BlockMined { from, block }) => {
        //                     println!("Received mined block");
        //                 },
        //                 _ => {
        //                     println!("Received invalid message");
        //                 }
        //             }
        //         }
        //     });
        // }

        // self.block_tx = Some(block_tx);

        // while let Some(mined_block) = block_rx.recv().await {
        //     let message = Message::BlockMined {
        //         from: endpoint.node_id(),
        //         block: mined_block.clone()
        //     };
        //
        //     self.sender.as_mut().unwrap().broadcast(message.to_vec().into()).await?;
        //
        //     println!("Sent mined block");
        // }

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
        loop {
            match receiver.try_next().await {
                Ok(Some(Event::Gossip(GossipEvent::Received(msg)))) => {
                    match Message::from_bytes(&msg.content) {
                        Ok(parsed) => println!("Message: {:?}", parsed),
                        Err(e) => eprintln!("Failed to parse message: {e}"),
                    }
                }
                Ok(Some(event)) => {
                    println!("Ignored event: {:?}", event);
                }
                Ok(None) => {
                    println!("Receiver closed");
                    break;
                }
                Err(e) => {
                    eprintln!("Error from try_next: {e}");
                    break;
                }
            }
        }

        Ok(())
    }
}