use crate::wallet::Wallet;

pub struct User {
    pub username: String,
    pub wallet: Wallet
}

#[derive(sqlx::FromRow, Debug)]
pub struct UserDB {
    pub username: String,
    pub public_key: String,
    pub private_key: String,
    pub address: String
}

pub fn convert_to_user(user_db: UserDB) -> User {
    let wallet = Wallet::load(user_db.private_key, user_db.public_key, user_db.address);

    User { wallet, username: user_db.username }
}