use std::sync::Arc;
use tokio::sync::Mutex;
use MockChain::args::args::Args;
use MockChain::args::mode::{Mode, ModeArgs};
use MockChain::args::node_type::NodeType;
use MockChain::block::Block;
use MockChain::database::connection::Connection;
use MockChain::network::node::Node;

pub async fn wait_for_genesis(node: Arc<Mutex<Node>>) -> Block {
    let mut node = node.lock().await;
    while node.blockchain.get_length() == 0 {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    node.blockchain.get_latest_block().clone()
}

pub fn create_node(node_address: String) -> Arc<Mutex<Node>> {
    Arc::new(Mutex::new(Node::new(node_address)))
}

pub async fn create_db_connection() -> Arc<Connection> {
    Arc::new(Connection::new().await)
}

pub fn create_open_node_args(node_address: String) -> Args {
    let mode = Mode::OPEN { node_address };
    let full_node = NodeType::FULL(ModeArgs { mode });

    Args { node_type: full_node }
}

pub fn create_join_node_args(node_address: String, peer_address: String) -> Args {
    let mode = Mode::JOIN { node_address, peer_address };
    let full_node = NodeType::FULL(ModeArgs { mode });

    Args { node_type: full_node }
}