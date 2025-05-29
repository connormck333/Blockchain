use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionRequest {
    pub sender_public_key: String,
    pub recipient_address: String,
    pub id: String,
    pub timestamp: i64,
    pub amount: u64,
    pub signature: String
}