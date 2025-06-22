mod common;

#[cfg(test)]
mod tests {
    use MockChain::init::test_init;
    use crate::common::utils::{create_db_connection, create_join_node_args, create_node, create_open_node_args, wait_for_genesis};

    #[tokio::test]
    async fn happy_path_init_2_nodes() {
        dotenv::dotenv().ok();

        let node_address_1 = "127.0.0.1:8080".to_string();
        let node_1 = create_node(node_address_1.clone());
        let db_connection = create_db_connection().await;
        let args_1 = create_open_node_args(node_address_1.clone());

        let node_address_2 = "127.0.0.1:8081".to_string();""
        let node_2 = create_node(node_address_2.clone());
        let args_2 = create_join_node_args(node_address_2, node_address_1.clone());

        test_init(node_1.clone(), db_connection.clone(), args_1.clone()).await.expect("Error initializing node 1");
        test_init(node_2.clone(), db_connection.clone(), args_2.clone()).await.expect("Error initializing node 2");

        let genesis_block_1 = wait_for_genesis(node_1.clone()).await;

        assert_eq!(genesis_block_1.index, 0);
        assert_eq!(genesis_block_1.previous_block_hash, "0");
        assert_eq!(genesis_block_1.miner_address, node_1.lock().await.wallet.address);
        assert!(genesis_block_1.transactions.is_empty());
        assert!(!genesis_block_1.hash.is_empty());
    }
}