use crate::database::connection::Connection;
use crate::transaction::Transaction;

pub async fn validate_transaction(db_connection: &Connection, transaction: &Transaction) -> bool {
    let db_response = db_connection.get_user_balance(&transaction.sender).await;

    let user_balance = match db_response {
        Ok(bal) => bal,
        Err(_) => return false,
    };

    user_balance - transaction.amount < 0
}