#[derive(sqlx::FromRow)]
pub struct MiningReward {
    pub amount: u64,
    pub recipient_address: String,
    pub block_unlocked_at: u64
}

impl MiningReward {
    pub fn new(amount: u64, recipient_address: String, block_unlocked_at: u64) -> Self {
        Self {
            amount,
            recipient_address,
            block_unlocked_at
        }
    }
}