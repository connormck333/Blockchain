use crate::block::Block;
use crate::blockchain::Blockchain;

pub struct Node {
    pub name: String,
    pub blockchain: Blockchain
}

impl Node {
    pub fn new(name: &str, difficulty: usize) -> Self {
        Self {
            name: name.to_string(),
            blockchain: Blockchain::new(difficulty)
        }
    }

    pub fn receive_block(&mut self, block: Block) -> bool {
        if self.blockchain.is_valid_new_block(&block) {
            self.blockchain.chain.push(block);
            println!("{} accepted new block", self.name);
            true
        } else {
            println!("{} rejected the block", self.name);
            false
        }
    }
}