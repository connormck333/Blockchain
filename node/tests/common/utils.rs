use MockChain::block::Block;

pub fn create_genesis_block() -> Block {
    Block::new(
        0,
        "0".to_string(),
        vec![],
        "miner_address".to_string()
    )
}