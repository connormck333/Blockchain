use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;

pub fn hash_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hashed = argon2.hash_password(password.as_bytes(), &salt);
    if hashed.is_err() {
        return Err(anyhow::anyhow!("Failed to hash password."));
    }

    Ok(hashed.unwrap().to_string())
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hashed_password);
    if parsed_hash.is_ok() {
        let verified = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash.unwrap())
            .is_ok();

        return verified;
    }

    false
}