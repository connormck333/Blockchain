use sqlx::{Error, Pool, Postgres};
use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;
use crate::constants::MINING_REWARD_AMOUNT;
use crate::database::operations::DatabaseOperations;
use crate::database::structs::recipient_address::RecipientAddress;
use crate::database::structs::user_balance::UserBalance;
use crate::mining::mining_reward::MiningReward;

pub struct Connection {
    pub pool: Pool<Postgres>,
    db_name: String
}

#[async_trait::async_trait]
impl DatabaseOperations for Connection {
    async fn create_user(&self, user_address: String, balance: u64) -> bool {
        self.create_user(user_address, balance).await
    }

    async fn get_user_balance(&self, user_address: &String) -> anyhow::Result<u64> {
        self.get_user_balance(user_address).await
    }

    async fn create_user_if_not_exists(&self, user_address: &String, balance: u64) -> bool {
        self.create_user_if_not_exists(user_address, balance).await
    }

    async fn update_user_balance(&self, user_address: String, amount: i64) -> bool {
        self.update_user_balance(user_address, amount).await
    }

    async fn save_mining_reward(&self, mining_reward: MiningReward) -> bool {
        self.save_mining_reward(mining_reward).await
    }

    async fn get_mining_reward_at_block_index(&self, block_index: u64) -> anyhow::Result<RecipientAddress> {
        self.get_mining_reward_at_block_index(block_index).await
    }

    async fn create_user_and_update_balance(&self, recipient_address: String, amount: i64) {
        self.create_user_and_update_balance(recipient_address, amount).await;
    }

    async fn drop_database(&self) {
        self.drop_database().await;
    }

    fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}

impl Connection {

    pub async fn new() -> Self {
        let db_name = format!("devconnor_blockchain_{}", Uuid::new_v4());
        let db_url = Self::get_db_url(db_name.clone());

        Self::create_database(db_url.clone()).await;

        let db_pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(db_url.as_str())
            .await
            .expect("Database pool could not be created");

        sqlx::migrate!().run(&db_pool).await.expect("Migrating database failed");

        Self {pool: db_pool, db_name}
    }

    fn get_db_url(db_name: String) -> String {
        let db_username = std::env::var("POSTGRES_USERNAME").expect("Database username not set as environment variable");
        let db_password = std::env::var("POSTGRES_PASSWORD").expect("Database password not set as environment variable");
        let db_host = std::env::var("POSTGRES_HOST").expect("Database host not set as environment variable");

        println!("Database url: postgresql://{}:{}@{}/{}", db_username, db_password, db_host, db_name);

        format!("postgresql://{}:{}@{}/{}", db_username, db_password, db_host, db_name)
    }

    async fn create_database(db_name: String) {
        Postgres::create_database(db_name.as_str()).await.expect("Database creation failed");

        println!("Database \"{}\" created.", db_name);
    }

    pub async fn create_user(&self, user_address: String, balance: u64) -> bool {
        let db_response = sqlx::query(
            r#"
            INSERT INTO users (address, balance)
            VALUES ($1, $2)
            "#
        )
        .bind(user_address)
        .bind(balance as i64)
        .execute(&self.pool)
        .await;

        if db_response.is_err() {
            println!("DB ERROR: {}", db_response.unwrap_err());
            return false
        }

        db_response.is_ok()
    }

    pub async fn get_user_balance(&self, user_address: &String) -> anyhow::Result<u64> {
        let balance_retrieved: Result<UserBalance, Error> = sqlx::query_as(
            "SELECT balance FROM USERS WHERE address = $1"
        )
        .bind(user_address)
        .fetch_one(&self.pool)
        .await;

        match balance_retrieved {
            Ok(user_balance) => {
                println!("User balance found: {}", user_balance.balance);
                Ok(user_balance.balance as u64)
            },
            Err(e) => {
                println!("{}", e.to_string());
                Err(anyhow::anyhow!("User not found"))
            }
        }
    }

    pub async fn create_user_if_not_exists(&self, user_address: &String, balance: u64) -> bool {
        let user_exists = self.get_user_balance(user_address).await;
        if user_exists.is_ok() {
            return true;
        }

        self.create_user(user_address.to_string(), balance).await;
        false
    }

    pub async fn update_user_balance(&self, user_address: String, amount: i64) -> bool {
        let db_response = sqlx::query(
            r#"
            UPDATE users
            SET balance = balance + $1
            WHERE address = $2
            "#
        )
        .bind(amount)
        .bind(user_address)
        .execute(&self.pool)
        .await;

        if db_response.is_err() {
            println!("ERROR when incrementing user balance: {}", db_response.unwrap_err());
            return false;
        }

        db_response.is_ok()
    }

    pub async fn save_mining_reward(&self, mining_reward: MiningReward) -> bool {
        println!("Saving mining reward to database");
        let db_response = sqlx::query(
            r#"
            INSERT INTO rewards (recipient_address, amount, block_unlocked_at)
            VALUES ($1, $2, $3)
            "#
        )
        .bind(mining_reward.recipient_address)
        .bind(mining_reward.amount as i64)
        .bind(mining_reward.block_unlocked_at as i64)
        .execute(&self.pool)
        .await;

        if db_response.is_err() {
            println!("ERROR when saving mining reward: {}", db_response.unwrap_err());
            return false
        }

        db_response.is_ok()
    }

    pub async fn get_mining_reward_at_block_index(&self, block_index: u64) -> anyhow::Result<RecipientAddress> {
        println!("Getting mining reward at block index: {}", block_index);
        let reward_retrieved: Result<RecipientAddress, Error> = sqlx::query_as(
            "SELECT recipient_address FROM rewards WHERE block_unlocked_at = $1"
        )
        .bind(block_index as i64)
        .fetch_one(&self.pool)
        .await;

        match reward_retrieved {
            Ok(address) => Ok(address),
            Err(e) => {
                println!("ERROR retrieving mining reward: {}", e);
                Err(anyhow::anyhow!("Mining reward not found"))
            }
        }
    }

    pub async fn create_user_and_update_balance(&self, recipient_address: String, amount: i64) {
        let user_exists = self.create_user_if_not_exists(&recipient_address, MINING_REWARD_AMOUNT).await;
        if user_exists {
            // Increment user balance if already exists in db
            // Otherwise, the balance will be saved on user creation
            self.update_user_balance(recipient_address, amount).await;
        }
    }

    pub async fn drop_database(&self) {
        self.pool.close().await;
        Postgres::drop_database(Self::get_db_url(self.db_name.clone()).as_str()).await.expect("Database drop failed");
        println!("Dropped database {}", self.db_name);
    }
}