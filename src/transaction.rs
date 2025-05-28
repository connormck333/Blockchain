use chrono::Utc;
use secp256k1::ecdsa::Signature;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use sha2::{Digest, Sha256};
use crate::utils::calculate_hash;

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub timestamp: i64,

    #[serde(skip_serializing, skip_deserializing)]
    pub id: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub signature: Option<Signature>
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: u64) -> Self {
        let mut transaction = Self {
            sender,
            recipient,
            amount,
            timestamp: Utc::now().timestamp(),
            id: "".to_string(),
            signature: None
        };

        transaction.id = transaction.create_hash();

        transaction
    }

    fn create_hash(&self) -> String {
        let serialized_tx = to_string(self).expect("Failed to serialize transaction");

        calculate_hash(serialized_tx)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Failed to serialize transaction")
    }

    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self.to_bytes());
        hasher.finalize().into()
    }
}