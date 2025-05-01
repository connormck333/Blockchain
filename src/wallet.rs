use ripemd::{Ripemd160, Digest};
use secp256k1::{Message, Secp256k1, SecretKey, PublicKey};
use secp256k1::ecdsa::Signature;
use secp256k1::rand::thread_rng;
use sha2::{Sha256, Digest as ShaDigest};
use crate::transaction::Transaction;

#[derive(Clone)]
pub struct Wallet {
    private_key: SecretKey,
    pub public_key: PublicKey,
    pub address: String
}

impl Wallet {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let (private_key, public_key) = Secp256k1::generate_keypair(&secp, &mut thread_rng());
        let address = Self::create_address_hash(&public_key);

        Wallet {
            private_key,
            public_key,
            address
        }
    }

    fn create_address_hash(public_key: &PublicKey) -> String {
        let mut sha_hasher = Sha256::new();
        sha_hasher.update(public_key.serialize());
        let sha_result = sha_hasher.finalize();

        let mut ripemd_hasher = Ripemd160::new();
        ripemd_hasher.update(sha_result);

        hex::encode(ripemd_hasher.finalize())
    }

    pub fn create_signature(&self, transaction: &Transaction) -> Signature {
        let secp = Secp256k1::new();

        let tx_hash = transaction.hash();
        let message = Message::from_digest(tx_hash);

        secp.sign_ecdsa(message, &self.private_key)
    }

    pub fn verify_signature(&self, transaction: &Transaction) -> bool {
        let secp = Secp256k1::verification_only();
        let tx_hash = transaction.hash();
        let message = Message::from_digest(tx_hash);

        secp.verify_ecdsa(message, &transaction.signature.unwrap(), &self.public_key).is_ok()
    }

    pub fn get_public_key(&self) -> String {
        hex::encode(self.public_key.serialize())
    }
}