use hex::encode;
use sha2::{Digest, Sha256};

pub fn calculate_hash(serialized_data: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(serialized_data.as_bytes());
    let result = hasher.finalize();

    encode(result)
}