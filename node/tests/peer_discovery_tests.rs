mod common;

#[cfg(test)]
mod tests {
    use MockChain::init::test_init;
    use crate::common::utils::{create_join_node_args, create_mocked_database, create_node, create_open_node_args, extract_peer_addresses_from_node, init_logger, wait_for_block_at_index, wait_for_genesis};

    fn setup() {
        dotenv::dotenv().ok();
        init_logger();
    }

    #[tokio::test]
    async fn should_connect_and_exchange_genesis_block() {
        setup();

        let db = create_mocked_database();

        let node_address_1 = "127.0.0.1:8080".to_string();
        let node_1 = create_node(node_address_1.clone());
        let args_1 = create_open_node_args(node_address_1.clone());

        let node_address_2 = "127.0.0.1:8081".to_string();
        let node_2 = create_node(node_address_2.clone());
        let args_2 = create_join_node_args(node_address_2.clone(), node_address_1);

        // Spawn 2 nodes
        tokio::spawn(test_init(node_1.clone(), db.clone(), args_1.clone()));
        tokio::spawn(test_init(node_2.clone(), db.clone(), args_2.clone()));

        // Wait for genesis block to be exchanged
        let genesis_block_1 = wait_for_genesis(node_1.clone()).await;
        let genesis_block_2 = wait_for_genesis(node_2.clone()).await;

        // Ensure both nodes have the same genesis block
        assert!(genesis_block_1.equals(&genesis_block_2), "Genesis blocks do not match");
    }

    #[tokio::test]
    async fn should_connect_and_exchange_full_chain() {
        setup();

        let db = create_mocked_database();

        let node_address_1 = "127.0.0.1:8080".to_string();
        let node_1 = create_node(node_address_1.clone());
        let args_1 = create_open_node_args(node_address_1.clone());

        let node_address_2 = "127.0.0.1:8081".to_string();
        let node_2 = create_node(node_address_2.clone());
        let args_2 = create_join_node_args(node_address_2.clone(), node_address_1.clone());

        // Spawn 2 nodes
        tokio::spawn(test_init(node_1.clone(), db.clone(), args_1.clone()));
        tokio::spawn(test_init(node_2.clone(), db.clone(), args_2.clone()));

        // Wait for blockchain to reach length of 5
        let node_1_block_5 = wait_for_block_at_index(node_1.clone(), 5).await;
        let node_2_block_5 = wait_for_block_at_index(node_2.clone(), 5).await;

        // Create third node and initialize it
        let node_address_3 = "127.0.0.1:8082".to_string();
        let node_3 = create_node(node_address_3.clone());
        let args_3 = create_join_node_args(node_address_3.clone(), node_address_1.clone());
        tokio::spawn(test_init(node_3.clone(), db.clone(), args_3.clone()));

        // Wait for third node to receive the full chain
        let node_3_block_5 = wait_for_block_at_index(node_3.clone(), 5).await;

        println!("{}", node_1_block_5);
        println!("{}", node_2_block_5);
        println!("{}", node_3_block_5);

        assert!(node_3_block_5.equals(&node_1_block_5));
        assert!(node_3_block_5.equals(&node_2_block_5));
    }

    #[tokio::test]
    async fn should_discover_peers_on_first_connection() {
        setup();

        let db = create_mocked_database();

        let node_address_1 = "127.0.0.1:8080".to_string();
        let node_1 = create_node(node_address_1.clone());
        let args_1 = create_open_node_args(node_address_1.clone());

        let node_address_2 = "127.0.0.1:8081".to_string();
        let node_2 = create_node(node_address_2.clone());
        let args_2 = create_join_node_args(node_address_2.clone(), node_address_1.clone());

        // Spawn first 2 nodes
        tokio::spawn(test_init(node_1.clone(), db.clone(), args_1.clone()));
        tokio::spawn(test_init(node_2.clone(), db.clone(), args_2.clone()));

        // Wait for nodes to initialize and discover each other
        wait_for_genesis(node_2.clone()).await;

        // Ensure both nodes have discovered each other
        let peers_1 = extract_peer_addresses_from_node(node_1.clone()).await;
        let peers_2 = extract_peer_addresses_from_node(node_2.clone()).await;
        assert!(peers_1.contains(&node_address_2));
        assert!(peers_2.contains(&node_address_1));

        // Create third node and initialize it
        let node_address_3 = "127.0.0.1:8090".to_string();
        let node_3 = create_node(node_address_3.clone());
        let args_3 = create_join_node_args(node_address_3.clone(), node_address_1.clone());
        tokio::spawn(test_init(node_3.clone(), db.clone(), args_3.clone()));

        // Wait for third node to discover the first two nodes
        wait_for_genesis(node_3.clone()).await;

        // Assert that the third node has discovered both first and second nodes
        let peers_3 = extract_peer_addresses_from_node(node_3.clone()).await;
        assert!(peers_3.contains(&node_address_1));
        assert!(peers_3.contains(&node_address_2));
    }
}