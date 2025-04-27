use uuid::Uuid;
use crate::block::Block;
use crate::node::Node;
use crate::simulator::log_panel::LogPanel;

pub struct Network {
    pub nodes: Vec<Node>
}

impl Default for Network {
    fn default() -> Self {
        Self {
            nodes: Vec::new()
        }
    }
}

impl Network {
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn broadcast_block(&mut self, block: Block, sender_node: Uuid, log_panel: &mut LogPanel) {
        println!("Broadcasting block to all nodes");
        for node in self.nodes.iter_mut() {
            if node.id == sender_node {
                continue;
            }

            let valid_block: bool = node.receive_block(block.clone());

            log_panel.add_log(
                format!("{} {} the block", node.name.clone(), Self::get_action_message(valid_block)),
                format!("Node {} block with id {}", Self::get_action_message(valid_block), block.index),
                node.name.clone()
            );
        }
    }

    pub fn get_node_by_id(&mut self, id: Uuid) -> &mut Node {
        self.nodes
            .iter_mut()
            .find(|n| n.id == id)
            .unwrap()
    }

    fn get_action_message(valid_block: bool) -> String {
        if valid_block {"accepted".to_string()} else {"declined".to_string()}
    }
}