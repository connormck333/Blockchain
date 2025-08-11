use std::str::FromStr;
use secp256k1::ecdsa::Signature;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use crate::server::request::transaction::TransactionRequest;

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
    pub fn load(transaction_data: TransactionRequest) -> Self {
        Self {
            sender: transaction_data.sender_public_key,
            recipient: transaction_data.recipient_address,
            amount: transaction_data.amount,
            timestamp: transaction_data.timestamp,
            id: transaction_data.id,
            signature: Some(Signature::from_str(transaction_data.signature.as_str()).unwrap())
        }
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