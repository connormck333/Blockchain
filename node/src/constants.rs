pub const MINING_REWARD_DELAY: u64 = 5;
pub const MINING_REWARD_AMOUNT: u64 = 50;

#[cfg(any(test, feature = "mock"))]
pub const BLOCKCHAIN_DIFFICULTY: usize = 4;
#[cfg(not(any(test, feature = "mock")))]
pub const BLOCKCHAIN_DIFFICULTY: usize = 5;