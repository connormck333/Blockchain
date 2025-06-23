use crate::database::operations::DbOperations;
use crate::transaction::Transaction;
use crate::wallet::Wallet;

pub struct Validator {
    pub db: DbOperations
}

impl Validator {
    pub fn new(db: DbOperations) -> Self {
        Self {
            db
        }
    }

    pub async fn validate_transaction(&self, transaction: &Transaction) -> bool {
        let sender_address = Wallet::derive_address_hash_from_string(&transaction.sender);
        match self.db.get_user_balance(&sender_address).await {
            Ok(user_balance) => {
                println!("Transaction user balance valid: {}", user_balance >= transaction.amount);
                user_balance >= transaction.amount
            },
            Err(_) => false
        }
    }
}