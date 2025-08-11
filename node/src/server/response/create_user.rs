use serde::{Deserialize, Serialize};
use crate::chain::wallet::Wallet;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserResponse {
    pub public_key: String,
    pub private_key: String,
    pub address: String
}

impl CreateUserResponse {
    pub fn new(wallet: Wallet) -> Self {
        Self {
            public_key: wallet.get_public_key(),
            private_key: wallet.get_private_key(),
            address: wallet.address
        }
    }
}