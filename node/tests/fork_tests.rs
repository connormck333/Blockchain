mod common;

#[cfg(test)]
mod tests {
    use MockChain::block::Block;
    use MockChain::init::test_init;
    use crate::common::utils::{create_join_node_args, create_mocked_database, create_node, create_open_node_args, init_logger, mine_block, wait_for_block_at_index, wait_for_genesis};

    #[tokio::test]
    async fn should_detect_fork_and_resolve() {
        dotenv::dotenv().ok();
        init_logger();

        let db = create_mocked_database();

        let node_address_1 = "127.0.0.1:8080".to_string();
        let node_1 = create_node(node_address_1.clone());
        let args_1 = create_open_node_args(node_address_1.clone());

        let node_address_2 = "127.0.0.1:8081".to_string();
        let node_2 = create_node(node_address_2.clone());
        let args_2 = create_join_node_args(node_address_2.clone(), node_address_1.clone());

        // Spawn the first node & create genesis block
        tokio::spawn(test_init(node_1.clone(), db.clone(), args_1.clone()));
        let genesis_block = wait_for_genesis(node_1.clone()).await;

        // Spawn the second node & create another forked block
        let mut forked_block = Block::new(1, genesis_block.hash.clone(), vec![], node_address_1.clone());
        mine_block(&mut forked_block);
        tokio::spawn(test_init(node_2.clone(), db.clone(), args_2.clone()));

        // Wait for genesis to be received by node 2 & add the forked block
        let genesis_block_2 = wait_for_genesis(node_2.clone()).await;
        node_2.lock().await.blockchain.add_block_to_chain(&forked_block);

        // Ensure both nodes have the same genesis block
        assert!(genesis_block.equals(&genesis_block_2));

        // Ensure block at index 2 are not the same due to fork
        let node_1_block_2 = wait_for_block_at_index(node_1.clone(), 2).await;
        let node_2_block_2 = wait_for_block_at_index(node_2.clone(), 2).await;
        assert!(!node_1_block_2.equals(&node_2_block_2));

        // Ensure fork gets resolved
        let node_1_block_6 = wait_for_block_at_index(node_1.clone(), 6).await;
        let node_2_block_6 = wait_for_block_at_index(node_2.clone(), 6).await;

        println!("Node 1 Block 6: {:?}", node_1_block_6);
        println!("Node 2 Block 6: {:?}", node_2_block_6);

        assert!(node_1_block_6.equals(&node_2_block_6), "Fork was not resolved correctly");
    }
}