use std::sync::{Arc, Once};
use tokio::sync::Mutex;
use MockChain::args::args::Args;
use MockChain::args::mode::{Mode, ModeArgs};
use MockChain::args::node_type::NodeType;
use MockChain::block::Block;
use MockChain::database::connection::Connection;
use MockChain::database::operations::{DbOperations, MockDatabaseOperations};
use MockChain::database::structs::recipient_address::RecipientAddress;
use MockChain::network::node::Node;

static INIT: Once = Once::new();

pub fn init_logger() {
    INIT.call_once(|| {
        env_logger::builder()
            .is_test(true)
            .init();
    });
}

pub async fn wait_for_genesis(node: Arc<Mutex<Node>>) -> Block {
    wait_for_block_at_index(node, 0).await
}

pub async fn wait_for_block_at_index(node: Arc<Mutex<Node>>, index: usize) -> Block {
    log::info!("Waiting for block at index {} to be mined...", index);
    while node.lock().await.blockchain.get_length() <= index {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        log::info!("Checking for block at index {}...", index);
    }

    node.lock().await.blockchain.chain[index].clone()
}

pub fn create_node(node_address: String) -> Arc<Mutex<Node>> {
    Arc::new(Mutex::new(Node::new(node_address)))
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

pub async fn extract_peer_addresses_from_node(node: Arc<Mutex<Node>>) -> Vec<String> {
    let node = node.lock().await;
    node.peers.iter().map(|peer| peer.1.address.clone()).collect()
}

pub fn create_mocked_database() -> Arc<MockDatabaseOperations> {
    let recipient = RecipientAddress { recipient_address: "recipient".to_string() };
    let mut mocked_db = MockDatabaseOperations::new();
    mocked_db.expect_get_user_balance().returning(|_| Ok(1000));
    mocked_db.expect_update_user_balance().returning(|_, _| true);
    mocked_db.expect_get_mining_reward_at_block_index().returning(move |_| Ok(recipient.clone()));
    mocked_db.expect_save_mining_reward().returning(|_| true);
    mocked_db.expect_create_user_and_update_balance().returning(|_, _| ());

    Arc::new(mocked_db)
}