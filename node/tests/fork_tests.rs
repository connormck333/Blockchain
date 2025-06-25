mod common;

#[cfg(test)]
mod tests {
    use MockChain::block::Block;
    use MockChain::network::node::Node;
    use crate::common::utils::mine_block;

    const NODE_ADDRESS_1: String = "127.0.0.1:8080".to_string();
    const NODE_ADDRESS_2: String = "127.0.0.1:8081".to_string();

    #[tokio::test]
    async fn should_detect_fork_and_resolve() {
        let mut node_1 = Node::new(NODE_ADDRESS_1);
        let mut node_2 = Node::new(NODE_ADDRESS_2);

        let genesis = node_1.blockchain.create_genesis_block(NODE_ADDRESS_1);
        node_2.blockchain.load_starting_block(genesis.clone());

        let mut forked_block_1 =  Block::new(1, genesis.hash.clone(), vec![], NODE_ADDRESS_1.clone());
        mine_block(&mut forked_block_1);
        node_1.blockchain.add_block_to_chain(&forked_block_1);

        let mut forked_block_2 = Block::new(1, genesis.hash.clone(), vec![], NODE_ADDRESS_2.clone());
        mine_block(&mut forked_block_2);
        node_2.blockchain.add_block_to_chain(&forked_block_2);

        // let mut block_3 = Block::new(1, )

    }
}