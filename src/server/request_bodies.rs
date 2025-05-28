use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserData {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionData {
    pub username: String,
    pub password: String,
    pub recipient: String,
    pub amount: u64
}