use std::hash::{DefaultHasher, Hash, Hasher};
use sqlx::{Executor, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

pub struct Connection {
    pool: Pool<Postgres>,
    db_name: String
}

impl Connection {

    pub async fn new(node_id: Uuid) -> Self {
        let db_name = format!("devconnor_blockchain_{}", Self::hash_node_id(node_id));
        let db_username = std::env::var("POSTGRES_USERNAME").expect("Database username not set as environment variable");
        let db_password = std::env::var("POSTGRES_PASSWORD").expect("Database password not set as environment variable");

        let db_url = format!("postgresql://{}:{}@localhost/postgres", db_username, db_password);

        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url.as_str())
            .await
            .expect("Database pool could not be created");

        let connection = Connection {pool: db_pool, db_name};
        connection.delete_db_if_exists().await.expect("Existing database could not be deleted");
        connection.create_database().await.expect("Creating database failed");

        connection
    }

    fn hash_node_id(node_id: Uuid) -> String {
        let mut hasher = DefaultHasher::new();
        node_id.hash(&mut hasher);

        format!("{:x}", hasher.finish())
    }

    async fn delete_db_if_exists(&self) -> anyhow::Result<()> {
        self.pool
            .execute(format!("DROP DATABASE IF EXISTS {}", self.db_name).as_str())
            .await?;

        println!("Dropped database with name {}", self.db_name);
        Ok(())
    }

    async fn create_database(&self) -> anyhow::Result<()> {
        self.pool
            .execute(format!("CREATE DATABASE {}", self.db_name).as_str())
            .await?;

        println!("Created database {}", self.db_name);
        Ok(())
    }
}