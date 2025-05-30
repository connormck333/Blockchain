use std::hash::{DefaultHasher, Hash, Hasher};
use eframe::egui::TextBuffer;
use sqlx::{Error, Executor, Pool, Postgres};
use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;
use crate::database::structs::user::UserDB;
use crate::database::structs::user_balance::UserBalance;
use crate::wallet::Wallet;

pub struct Connection {
    pub pool: Pool<Postgres>,
    db_name: String
}

impl Connection {

    pub async fn new(node_id: Uuid) -> Self {
        let db_name = format!("devconnor_blockchain_{}", Self::hash_node_id(node_id));
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

        format!("postgresql://{}:{}@localhost/{}", db_username, db_password, db_name)
    }

    fn hash_node_id(node_id: Uuid) -> String {
        let mut hasher = DefaultHasher::new();
        node_id.hash(&mut hasher);

        format!("{:x}", hasher.finish())
    }

    async fn create_database(db_name: String) {
        Postgres::create_database(db_name.as_str()).await.expect("Database creation failed");

        println!("Database \"{}\" created.", db_name);
    }

    pub async fn create_user(&self, wallet: &Wallet) -> bool {
        let db_response = sqlx::query(
            r#"
            INSERT INTO users (public_key, address, balance)
            VALUES ($1, $2, 10)
            "#
        )
        .bind(wallet.get_public_key())
        .bind(wallet.address.clone())
        .execute(&self.pool)
        .await;

        if db_response.is_err() {
            println!("DB ERROR: {}", db_response.unwrap_err());
            return false
        }

        db_response.is_ok()
    }

    pub async fn get_user(&self, public_key: String) -> anyhow::Result<Wallet> {
        println!("Finding user with key: {}", public_key);
        let user_retrieved: Result<UserDB, Error> = sqlx::query_as(
            "SELECT * FROM USERS WHERE public_key = $1"
        )
        .bind(public_key)
        .fetch_one(&self.pool)
        .await;

        match user_retrieved {
            Ok(user_wallet) => Ok(Wallet::load(user_wallet.public_key, user_wallet.address)),
            Err(e) => {
                println!("{}", e.to_string());
                Err(anyhow::anyhow!("User not found"))
            }
        }
    }

    pub async fn get_user_balance(&self, public_key: &String) -> anyhow::Result<u64> {
        let balance_retrieved: Result<UserBalance, Error> = sqlx::query_as(
            "SELECT balance FROM USERS WHERE public_key = $1"
        )
        .bind(public_key)
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

    pub async fn drop_database(&self) {
        self.pool.close().await;
        Postgres::drop_database(Self::get_db_url(self.db_name.clone()).as_str()).await.expect("Database drop failed");
        println!("Dropped database {}", self.db_name);
    }
}