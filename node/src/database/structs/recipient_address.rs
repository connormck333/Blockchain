#[derive(sqlx::FromRow, Clone)]
pub struct RecipientAddress {
    pub recipient_address: String
}