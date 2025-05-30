use crate::database::connection::Connection;
use crate::transaction::Transaction;

pub async fn validate_transaction(db_connection: &Connection, transaction: &Transaction) -> bool {
    match db_connection.get_user_balance(&transaction.sender).await {
        Ok(user_balance) => {
            println!("User balance: {}", user_balance);
            user_balance >= transaction.amount
        },
        Err(_) => false,
    }
}