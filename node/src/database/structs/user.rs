#[derive(sqlx::FromRow, Debug)]
pub struct UserDB {
    pub public_key: String,
    pub address: String
}