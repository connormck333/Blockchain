use std::str::FromStr;
use std::sync::{Arc};
use std::sync::atomic::AtomicBool;
use tokio::sync::Mutex;
use futures_lite::StreamExt;
use iroh::{Endpoint, NodeAddr, NodeId};
use iroh::protocol::Router;
use iroh_gossip::net::{Event, Gossip, GossipEvent, GossipReceiver, GossipSender};
use iroh_gossip::proto::TopicId;
use crate::network::message::Message;
use crate::network::ticket::Ticket;
use crate::args::args::Args;
use crate::args::mode::Mode;
use crate::network::message_handler::handle_incoming_message;
use crate::network::node::Node;
use crate::database::validator::Validator;
use crate::mining_tasks::spawn_mining_loop;

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
            opening_node: matches!(args.node_type.get_mode(), Mode::OPEN)
        }
    }

    pub async fn connect(&mut self, node: Arc<Mutex<Node>>, validator: Arc<Validator>) -> anyhow::Result<()> {
        let endpoint = self.setup_endpoint().await;
        let (sender, receiver) = self.join_network(&endpoint).await;

        if self.opening_node {
            self.send_genesis_block(&sender, &node, endpoint.node_id()).await;
        }

        spawn_mining_loop(sender.clone(), node.clone(), self.mining_active.clone(), validator.db_connection.clone(), endpoint.node_id());

        tokio::spawn(Self::subscribe_loop(receiver, node.clone(), self.mining_active.clone(), validator.clone()));

        Ok(())
    }

    fn get_topic_and_nodes(args: Args) -> (TopicId, Vec<NodeAddr>) {
        match &args.node_type.get_mode() {
            Mode::OPEN => {
                let topic = TopicId::from_bytes(rand::random());
                println!("> Opening chatroom for topic {topic}");
                (topic, vec![])
            }
            Mode::JOIN { ticket } => {
                let ticket = Ticket::from_str(&ticket).unwrap();
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
        println!("> Node id: {}", endpoint.node_id());

        endpoint
    }

    async fn join_network(&mut self, endpoint: &Endpoint) -> (GossipSender, GossipReceiver) {
        let node_ids = self.connected_nodes.clone().iter().map(|p| p.node_id).collect();
        if self.connected_nodes.is_empty() {
            println!("> Waiting for nodes to join");
        } else {
            println!("> Joining nodes: {}", self.connected_nodes.len());
            for node in self.connected_nodes.clone().into_iter() {
                endpoint.add_node_addr(node).unwrap();
            }
        }

        let (sender, receiver) = self.gossip.as_mut().unwrap().subscribe_and_join(self.topic_id, node_ids).await.unwrap().split();
        println!("> Connected");

        (sender, receiver)
    }

    async fn send_genesis_block(&mut self, sender: &GossipSender, node: &Arc<Mutex<Node>>, node_id: NodeId) {
        let miner_address = node.lock().await.wallet.address.clone();
        let genesis_block = node.lock().await.blockchain.create_genesis_block(miner_address);
        let message = Message::GenesisBlock {
            from: node_id,
            genesis_block
        };

        Self::send_message(message, &sender).await;
        println!("> Sent genesis block message");
    }

    async fn subscribe_loop(
        mut receiver: GossipReceiver,
        node: Arc<Mutex<Node>>,
        mining_flag: Arc<AtomicBool>,
        validator: Arc<Validator>
    ) -> anyhow::Result<()> {
        loop {
            match receiver.try_next().await {
                Ok(Some(Event::Gossip(GossipEvent::Received(msg)))) => {
                    handle_incoming_message(node.clone(), mining_flag.clone(), validator.clone(), msg).await;
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

    async fn send_message(message: Message, sender: &GossipSender) {
        let bytes = message.to_vec().into();
        let _ = sender.broadcast(bytes).await;
    }
}