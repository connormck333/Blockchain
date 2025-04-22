mod block;
mod transaction;
mod blockchain;
mod utils;
mod node;
mod network;

use crate::block::Block;
use crate::network::Network;
use crate::node::Node;

fn main() {
    let mut network = Network::new();

    network.add_node(Node::new("Node-A", 3));
    network.add_node(Node::new("Node-B", 3));
    network.add_node(Node::new("Node-C", 3));

    let mut miner = Node::new("Miner-1", 3);

    let transactions = miner.blockchain.get_mempool().clone();
    let previous_hash = miner.blockchain.get_latest_block().hash.clone();

    let mut block = Block::new(miner.blockchain.get_chain().len() as u64, previous_hash, transactions, 3);

    block.mine();

    miner.blockchain.add_block_to_chain(block.clone());

    network.broadcast_block(block);
}
