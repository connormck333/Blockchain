use std::hash::{DefaultHasher, Hash, Hasher};
use eframe::egui::TextBuffer;
use sqlx::{Executor, Pool, Postgres};
use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;
use crate::database::structs::user::{convert_to_user, User, UserDB};
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

    pub async fn create_user(&self, username: &str, password: &str, wallet: Wallet) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO users (username, password, public_key, private_key, address)
            VALUES ($1, $2, $3, $4, $5)
            "#
        )
        .bind(username)
        .bind(password)
        .bind(wallet.get_public_key())
        .bind(wallet.get_private_key())
        .bind(wallet.address)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_user(&self, username: &str, password: &str) -> anyhow::Result<User> {
        let user_retrieved: UserDB = sqlx::query_as(
            "SELECT username, private_key, public_key, address FROM USERS
            WHERE username = $1 AND password = $2"
        )
        .bind(username)
        .bind(password)
        .fetch_one(&self.pool)
        .await?;

        Ok(convert_to_user(user_retrieved))
    }

    pub async fn drop_database(&self) {
        self.pool.close().await;
        Postgres::drop_database(Self::get_db_url(self.db_name.clone()).as_str()).await.expect("Database drop failed");
        println!("Dropped database {}", self.db_name);
    }
}