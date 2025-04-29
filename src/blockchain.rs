use crate::block::Block;

/*
    Blockchain is a shared, immutable digital ledger, enabling the recording of transactions
    and the tracking of assets within a business network and providing a single source of truth.
*/

#[derive(Clone)]
pub struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let genesis_block = Self::create_genesis_block(difficulty);
        Self {
            chain: vec![genesis_block],
            difficulty
        }
    }

    pub fn is_valid_new_block(&self, new_block: &Block) -> bool {
        self.validate_block_data(new_block) &&
            self.validate_block_transactions(new_block)
    }

    pub fn add_block_to_chain(&mut self, new_block: Block) -> bool {
        if !self.is_valid_new_block(&new_block) {
            return false;
        }

        self.chain.push(new_block);
        true
    }

    pub fn get_chain(&self) -> &Vec<Block> {
        &self.chain
    }

    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    fn validate_block_data(&self, new_block: &Block) -> bool {
        let last_block = self.chain.last().unwrap();

        new_block.previous_block_hash == last_block.hash &&
            new_block.index == last_block.index + 1 &&
            new_block.hash.starts_with(&"0".repeat(self.difficulty)) &&
            new_block.hash == new_block.create_hash()
    }

    fn validate_block_transactions(&self, block: &Block) -> bool {
        for transaction in &block.transactions {
            let sender = transaction.sender.clone();
            let sender_funds: i64 = self.get_user_funds(sender);
            let remaining_funds: i64 = sender_funds - transaction.amount as i64;

            if remaining_funds < 0 {
                return false;
            }
        }

        true
    }

    fn get_user_funds(&self, user_address: String) -> i64 {
        let mut funds: i64 = 0;
        for block in &self.chain {
            for transaction in &block.transactions {
                if transaction.sender == user_address {
                    funds -= transaction.amount as i64;
                } else if transaction.recipient == user_address {
                    funds += transaction.amount as i64;
                }
            }
        }

        funds
    }

    fn create_genesis_block(difficulty: usize) -> Block {
        Block::new(0, "0".to_string(), Vec::new(), difficulty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let blockchain = Blockchain::new(2);
        assert_eq!(blockchain.difficulty, 2);
        assert_eq!(blockchain.chain.len(), 1);

        let genesis = blockchain.get_latest_block();
        assert_eq!(genesis.difficulty, 2);
        assert!(genesis.transactions.is_empty());
        assert_eq!(genesis.index, 0);
        assert_eq!(genesis.previous_block_hash, "0");
        assert_eq!(genesis.hash, "");
        assert_eq!(genesis.nonce, 0);
    }

    #[test]
    fn test_is_valid_new_block_success() {
        let blockchain = Blockchain::new(2);
        let prev_block = blockchain.get_latest_block();
        let mut new_block = Block::new(prev_block.index + 1, prev_block.hash.clone(), vec![], 2);

        new_block.mine();

        let is_valid = blockchain.is_valid_new_block(&new_block);

        assert!(is_valid);
    }

    #[test]
    fn test_is_valid_new_block_invalid_prev_hash() {
        let blockchain = Blockchain::new(2);
        let prev_block = blockchain.get_latest_block();
        let mut new_block = Block::new(prev_block.index + 1, "invalidHash".to_string(), vec![], 2);

        new_block.mine();

        let is_valid = blockchain.is_valid_new_block(&new_block);

        assert!(!is_valid);
    }

    #[test]
    fn test_is_valid_new_block_invalid_index() {
        let blockchain = Blockchain::new(2);
        let prev_block = blockchain.get_latest_block();
        let mut new_block = Block::new(prev_block.index + 10, prev_block.hash.clone(), vec![], 2);

        new_block.mine();

        let is_valid = blockchain.is_valid_new_block(&new_block);

        assert!(!is_valid);
    }

    #[test]
    fn test_is_valid_new_block_invalid_hash() {
        let blockchain = Blockchain::new(2);
        let prev_block = blockchain.get_latest_block();
        let mut new_block = Block::new(prev_block.index + 1, prev_block.hash.clone(), vec![], 2);

        new_block.mine();
        new_block.hash += "invalid";

        let is_valid = blockchain.is_valid_new_block(&new_block);

        assert!(!is_valid);
    }

    #[test]
    fn test_is_valid_new_block_invalid_hash_prefix() {
        let blockchain = Blockchain::new(2);
        let prev_block = blockchain.get_latest_block();
        let mut new_block = Block::new(prev_block.index + 1, prev_block.hash.clone(), vec![], 2);

        new_block.mine();
        new_block.hash = new_block.hash[1..].to_string();

        let is_valid = blockchain.is_valid_new_block(&new_block);

        assert!(!is_valid);
    }

    #[test]
    fn test_get_chain() {
        let mut blockchain = Blockchain::new(2);
        let prev_block = blockchain.get_latest_block().clone();
        let new_block = Block::new(prev_block.index + 1, prev_block.hash.clone(), vec![], 2);

        blockchain.add_block_to_chain(new_block.clone());

        let chain = blockchain.get_chain();

        assert_eq!(chain.len(), 2);
        assert_eq!(chain[0].index, prev_block.index);
        assert_eq!(chain[1].index, new_block.index);
    }

    #[test]
    fn test_get_latest_block() {
        let mut blockchain = Blockchain::new(2);
        let prev_block = blockchain.get_latest_block().clone();
        let new_block = Block::new(prev_block.index + 1, prev_block.hash.clone(), vec![], 2);

        blockchain.add_block_to_chain(new_block.clone());

        let latest_block = blockchain.get_latest_block();

        assert_eq!(latest_block.index, new_block.index);
    }
}