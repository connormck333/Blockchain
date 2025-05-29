#[derive(sqlx::FromRow, Debug)]
pub struct UserBalance {
    pub balance: i64
}