use ripemd::{Ripemd160, Digest};
use secp256k1::{Message, Secp256k1, SecretKey, PublicKey};
use secp256k1::rand::rng;
use sha2::{Sha256, Digest as ShaDigest};
use crate::chain::transaction::Transaction;

#[derive(Clone)]
pub struct Wallet {
    pub private_key: Option<SecretKey>,
    pub public_key: PublicKey,
    pub address: String
}

impl Wallet {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let (private_key, public_key) = Secp256k1::generate_keypair(&secp, &mut rng());
        let address = Self::derive_address_hash(&public_key);

        Wallet {
            private_key: Some(private_key),
            public_key,
            address
        }
    }

    pub fn load(public_key_str: String, address: String) -> Self {
        let public_key = Self::public_key_from_hex(&public_key_str);

        Self {
            private_key: None,
            public_key,
            address
        }
    }

    pub fn load_from_public_key(public_key_str: String) -> Self {
        let public_key = Self::public_key_from_hex(public_key_str.as_str());
        let address = Self::derive_address_hash(&public_key);

        Self {
            private_key: None,
            public_key,
            address
        }
    }

    pub fn derive_address_hash_from_string(public_key: &String) -> String {
        Self::derive_address_hash(&Self::public_key_from_hex(public_key.as_str()))
    }

    pub fn derive_address_hash(public_key: &PublicKey) -> String {
        let mut sha_hasher = Sha256::new();
        sha_hasher.update(public_key.serialize());
        let sha_result = sha_hasher.finalize();

        let mut ripemd_hasher = Ripemd160::new();
        ripemd_hasher.update(sha_result);

        hex::encode(ripemd_hasher.finalize())
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

    pub fn get_private_key(&self) -> String {
        hex::encode(self.private_key.unwrap().secret_bytes())
    }

    fn public_key_from_hex(hex_str: &str) -> PublicKey {
        let bytes = hex::decode(hex_str).map_err(|_| secp256k1::Error::InvalidPublicKey).unwrap();

        PublicKey::from_slice(&bytes).expect(&format!("Invalid public key: {}", hex_str))
    }
}