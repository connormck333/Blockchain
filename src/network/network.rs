use std::str::FromStr;
use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::sync::Mutex;
use futures_lite::StreamExt;
use iroh::{Endpoint, NodeAddr, NodeId};
use iroh::protocol::Router;
use iroh_gossip::net::{Event, Gossip, GossipEvent, GossipReceiver, GossipSender};
use iroh_gossip::proto::TopicId;
use crate::block::Block;
use crate::network::command::Command;
use crate::network::message::Message;
use crate::network::ticket::Ticket;
use crate::network::args::Args;
use crate::network::node::Node;
use crate::utils::mine_block;

pub struct Network {
    connected_nodes: Vec<NodeAddr>,
    topic_id: TopicId,
    gossip: Option<Gossip>,
    router: Option<Router>,
    mining_active: Arc<AtomicBool>,
    opening_node: bool
}

impl Network {

    pub fn new(args: Args) -> Self {
        let (topic_id, nodes) = Self::get_topic_and_nodes(args.clone());

        Self {
            connected_nodes: nodes,
            topic_id,
            gossip: None,
            router: None,
            mining_active: Arc::new(AtomicBool::new(true)),
            opening_node: matches!(args.command, Command::OPEN)
        }
    }

    pub async fn connect(&mut self, node: Arc<Mutex<Node>>) -> anyhow::Result<()> {
        let endpoint = self.setup_endpoint().await;
        let (sender, receiver) = self.join_network(&endpoint).await;

        if self.opening_node {
            self.send_genesis_block(&sender, &node, endpoint.node_id()).await;
        }

        Self::spawn_mining_loop(sender.clone(), node.clone(), self.mining_active.clone(), endpoint.node_id()).await?;

        tokio::spawn(Self::subscribe_loop(receiver, node.clone(), self.mining_active.clone()));

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
                (ticket.topic, ticket.peers)
            }
        }
    }

    async fn setup_endpoint(&mut self) -> Endpoint {
        let endpoint = Endpoint::builder().discovery_n0().bind().await.unwrap();

        self.gossip = Some(Gossip::builder()
            .spawn(endpoint.clone()).await
            .unwrap()
        );

        self.router = Some(Router::builder(endpoint.clone())
            .accept(iroh_gossip::ALPN, self.gossip.as_ref().unwrap().clone())
            .spawn().await
            .unwrap()
        );

        let ticket = {
            let me = endpoint.node_addr().await.unwrap();
            let nodes = vec![me];
            Ticket { topic: self.topic_id, peers: nodes }
        };
        println!("> Ticket to join us: {ticket}");

        endpoint
    }

    async fn join_network(&mut self, endpoint: &Endpoint) -> (GossipSender, GossipReceiver) {
        let node_ids = self.connected_nodes.clone().iter().map(|p| p.node_id).collect();
        if self.connected_nodes.is_empty() {
            println!("> Waiting for nodes to join");
        } else {
            println!("> Joining nodes: {}", self.connected_nodes.len());
            for node in self.connected_nodes.clone().into_iter() {
                endpoint.add_node_addr(node).unwrap()
            }
        }

        let (sender, receiver) = self.gossip.as_mut().unwrap().subscribe_and_join(self.topic_id, node_ids).await.unwrap().split();
        println!("> Connected");

        (sender, receiver)
    }

    async fn send_genesis_block(&mut self, sender: &GossipSender, node: &Arc<Mutex<Node>>, node_id: NodeId) {
        let genesis_block = node.clone().lock().await.blockchain.create_genesis_block();
        let message = Message::GenesisBlock {
            from: node_id,
            genesis_block
        };
        let bytes = message.to_vec().into();
        let _ = sender.broadcast(bytes).await;
        println!("> Sent genesis block message");
    }

    async fn subscribe_loop(mut receiver: GossipReceiver, node: Arc<Mutex<Node>>, mining_flag: Arc<AtomicBool>) -> anyhow::Result<()> {
        loop {
            match receiver.try_next().await {
                Ok(Some(Event::Gossip(GossipEvent::Received(msg)))) => {
                    match Message::from_bytes(&msg.content) {
                        Ok(parsed) => {
                            match parsed {
                                Message::GenesisBlock {from, genesis_block} => {
                                    tokio::time::sleep(Duration::from_millis(1000)).await;
                                    node.lock().await.blockchain.load_starting_block(genesis_block);
                                    println!("> Starting block received from {}", from);
                                    println!("> Starting mining...");
                                }
                                Message::BlockMined {from, block} => {
                                    if node.lock().await.receive_block(block) {
                                        mining_flag.store(false, Ordering::Relaxed);
                                        println!("Valid block received from {}... Stopping mining", from);
                                    } else {
                                        println!("Invalid block received from {}... Continuing to mine", from);
                                    }
                                }
                                _ => {
                                    println!("Unknown message received");
                                }
                            }
                        },
                        Err(e) => eprintln!("Failed to parse message: {e}"),
                    }
                }
                Ok(Some(Event::Gossip(GossipEvent::NeighborUp(new_node_id)))) => {

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

    async fn spawn_mining_loop(sender: GossipSender, node: Arc<Mutex<Node>>, mining_flag: Arc<AtomicBool>, node_id: NodeId) -> anyhow::Result<()> {
        tokio::spawn(async move {
            loop {
                if node.lock().await.blockchain.get_length() > 0 {
                    let mined_block: Option<Block> = tokio::task::spawn_blocking({
                        let cancel_flag = mining_flag.clone();
                        let node_inner = node.clone();
                        let transactions = node_inner.lock().await.mempool.clone();
                        let difficulty = node_inner.lock().await.difficulty.clone();
                        let blockchain_clone = node_inner.lock().await.blockchain.clone();
                        move || mine_block(
                            transactions,
                            difficulty,
                            blockchain_clone.get_latest_block().clone().hash,
                            blockchain_clone.get_length() as u64,
                            cancel_flag
                        )
                    }).await.unwrap();

                    if let Some(block) = mined_block {
                        node.lock().await.blockchain.add_block_to_chain(block.clone());
                        node.lock().await.mempool.clear();

                        let message = Message::BlockMined {
                            from: node_id,
                            block
                        };
                        let bytes = message.to_vec().into();
                        let _ = sender.broadcast(bytes).await;
                        println!("Sent mined block");
                    } else {
                        mining_flag.store(true, Ordering::Relaxed);
                    }
                }
            }
        });

        Ok(())
    }
}