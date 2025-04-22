use crate::block::Block;
use crate::node::Node;

pub struct Network {
    pub nodes: Vec<Node>
}

impl Network {

    pub fn new() -> Self {
        Self {
            nodes: Vec::new()
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn broadcast_block(&mut self, block: Block) {
        println!("Broadcasting block to all nodes");
        for node in self.nodes.iter_mut() {
            node.receive_block(block.clone());
        }
    }
}