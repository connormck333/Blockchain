use crate::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize
}

fn create_genesis_block() -> Block {
    Block::new(0, )
}