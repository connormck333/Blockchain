use std::sync::Arc;
use mockall::automock;
use crate::database::structs::recipient_address::RecipientAddress;
use crate::mining_reward::MiningReward;

pub type DbOperations = Arc<dyn DatabaseOperations + Send + Sync>;

#[cfg_attr(any(test, feature = "mock"), automock)]
#[async_trait::async_trait]
pub trait DatabaseOperations: Send + Sync {
    async fn create_user(&self, user_address: String, balance: u64) -> bool;
    async fn get_user_balance(&self, user_address: &String) -> anyhow::Result<u64>;
    async fn create_user_if_not_exists(&self, user_address: &String, balance: u64) -> bool;
    async fn update_user_balance(&self, user_address: String, amount: i64) -> bool;
    async fn save_mining_reward(&self, mining_reward: MiningReward) -> bool;
    async fn get_mining_reward_at_block_index(&self, block_index: u64) -> anyhow::Result<RecipientAddress>;
    async fn create_user_and_update_balance(&self, recipient_address: String, amount: i64);
    async fn drop_database(&self);
    fn get_pool(&self) -> &sqlx::Pool<sqlx::Postgres>;
}