use std::sync::Arc;
use crate::database::connection::Connection;
use crate::transaction::Transaction;
use crate::wallet::Wallet;

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
        let sender_address = Wallet::derive_address_hash_from_string(&transaction.sender);
        match self.db_connection.get_user_balance(&sender_address).await {
            Ok(user_balance) => {
                println!("User balance: {}", user_balance);
                user_balance >= transaction.amount
            },
            Err(_) => false,
        }
    }
}