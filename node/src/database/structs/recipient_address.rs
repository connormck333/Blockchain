#[derive(sqlx::FromRow)]
pub struct RecipientAddress {
    pub recipient_address: String
}