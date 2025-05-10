use std::str::FromStr;
use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
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
    gossip: Option<Gossip>,
    mining_active: Arc<AtomicBool>
}

impl Network {

    pub fn new(args: Args) -> Self {
        let (topic_id, nodes) = Self::get_topic_and_nodes(args.clone());

        Self {
            connected_nodes: nodes,
            topic_id,
            gossip: None,
            mining_active: Arc::new(AtomicBool::new(true))
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
            let mining_flag = self.mining_active.clone();
            tokio::spawn(async move {
                loop {
                    let mined_block: Option<Block> = tokio::task::spawn_blocking({
                        let node_inner = node_clone.clone();
                        let cancel_flag = mining_flag.clone();
                        move || node_inner.blocking_lock().mine_block(cancel_flag)
                    }).await.unwrap();

                        if let Some(block) = mined_block {
                            let message = Message::BlockMined {
                                from: endpoint.node_id(),
                                block
                            };
                            let bytes = message.to_vec().into();
                            println!("Sending mined block");
                            let _ = sender.broadcast(bytes).await;
                            println!("Sent mined block");
                        } else {
                            mining_flag.store(true, Ordering::Relaxed);
                        }
                }
            });
        }

        // Listens for incoming messages
        let mining_flag = self.mining_active.clone();
        tokio::spawn(Self::subscribe_loop(receiver, mining_flag));

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

    async fn subscribe_loop(mut receiver: GossipReceiver, mining_flag: Arc<AtomicBool>) -> anyhow::Result<()> {
        loop {
            match receiver.try_next().await {
                Ok(Some(Event::Gossip(GossipEvent::Received(msg)))) => {
                    match Message::from_bytes(&msg.content) {
                        Ok(parsed) => {
                            mining_flag.store(false, Ordering::Relaxed);
                            println!("Block received... Stopping mining");
                        },
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