mod common;

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use MockChain::database::operations::MockDatabaseOperations;
    use MockChain::database::structs::recipient_address::RecipientAddress;
    use MockChain::init::test_init;
    use crate::common::utils::{create_join_node_args, create_node, create_open_node_args, init_logger, wait_for_block_at_index, wait_for_genesis};

    #[tokio::test]
    async fn happy_path_init_2_nodes() {
        dotenv::dotenv().ok();
        init_logger();

        let recipient = RecipientAddress { recipient_address: "recipient".to_string() };
        let mut mocked_db = MockDatabaseOperations::new();
        mocked_db.expect_get_user_balance().returning(|_| Ok(1000));
        mocked_db.expect_update_user_balance().returning(|_, _| true);
        mocked_db.expect_get_mining_reward_at_block_index().returning(move |_| Ok(recipient.clone()));
        mocked_db.expect_save_mining_reward().returning(|_| true);

        let db = Arc::new(mocked_db);

        let node_address_1 = "127.0.0.1:8080".to_string();
        let node_1 = create_node(node_address_1.clone());
        let args_1 = create_open_node_args(node_address_1.clone());

        let node_address_2 = "127.0.0.1:8081".to_string();
        let node_2 = create_node(node_address_2.clone());
        let args_2 = create_join_node_args(node_address_2, node_address_1.clone());

        tokio::spawn(test_init(node_1.clone(), db.clone(), args_1.clone()));
        tokio::spawn(test_init(node_2.clone(), db.clone(), args_2.clone()));

        let genesis_block_1 = wait_for_genesis(node_1.clone()).await;
        let genesis_block_2 = wait_for_genesis(node_2.clone()).await;
        assert!(genesis_block_1.equals(&genesis_block_2));

        let node_1_block_5 = wait_for_block_at_index(node_1.clone(), 5).await;
        let node_2_block_5 = wait_for_block_at_index(node_2.clone(), 5).await;
        assert!(node_1_block_5.equals(&node_2_block_5));
    }
}