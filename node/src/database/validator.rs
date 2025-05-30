use std::sync::Arc;
use crate::database::connection::Connection;
use crate::transaction::Transaction;

pub struct Validator {
    pub db_connection: Arc<Connection>
}

impl Validator {
    pub fn new(db_connection: Arc<Connection>) -> Self {
        Self {
            db_connection
        }
    }

    pub async fn validate_transaction(&self, transaction: &Transaction) -> bool {
        match self.db_connection.get_user_balance(&transaction.sender).await {
            Ok(user_balance) => {
                println!("User balance: {}", user_balance);
                user_balance >= transaction.amount
            },
            Err(_) => false,
        }
    }
}