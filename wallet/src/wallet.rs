use ripemd::{Digest};
use secp256k1::{Message, Secp256k1, SecretKey, PublicKey};
use secp256k1::ecdsa::Signature;
use secp256k1::rand::rng;
use sha2::{Digest as ShaDigest};
use crate::transaction::Transaction;

#[derive(Clone)]
pub struct Wallet {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
    pub address: String
}

impl Default for Wallet {
    fn default() -> Self {
        let secp = Secp256k1::new();
        let (private_key, public_key) = Secp256k1::generate_keypair(&secp, &mut rng());

        Self {
            private_key,
            public_key,
            address: "".to_string()
        }
    }
}

impl Wallet {
    pub fn new(private_key_str: String, public_key_str: String, address: String) -> Self {
        let private_key = Self::private_key_from_hex(&private_key_str);
        let public_key = Self::public_key_from_hex(&public_key_str);

        Self {
            private_key,
            public_key,
            address
        }
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

    pub fn get_private_key(&self) -> String {
        hex::encode(self.private_key.secret_bytes())
    }

    pub fn public_key_from_hex(hex_str: &str) -> PublicKey {
        let bytes = hex::decode(hex_str).map_err(|_| secp256k1::Error::InvalidPublicKey).unwrap();

        PublicKey::from_slice(&bytes).expect(&format!("Invalid public key: {}", hex_str))
    }

    pub fn private_key_from_hex(hex_str: &str) -> SecretKey {
        let bytes = hex::decode(hex_str).unwrap();
        let key_bytes: [u8; 32] = bytes.try_into().map_err(|_| anyhow::anyhow!("Invalid key length")).unwrap();

        SecretKey::from_byte_array(key_bytes).expect("Invalid private key")
    }
}