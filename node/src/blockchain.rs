use crate::block::Block;
use crate::constants::BLOCKCHAIN_DIFFICULTY;
/*
    Blockchain is a shared, immutable digital ledger, enabling the recording of transactions
    and the tracking of assets within a business network and providing a single source of truth.
*/

#[derive(Clone)]
pub struct Blockchain {
    chain: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            chain: vec![]
        }
    }

    pub fn is_valid_new_block(&self, new_block: &Block) -> bool {
        if self.chain.len() == 0 {
            return true;
        }

        let last_block = self.chain.last().unwrap();
        println!("Checking new block validity, current length {}", self.chain.len());

        new_block.previous_block_hash == last_block.hash &&
            new_block.index == last_block.index + 1 &&
            new_block.hash.starts_with(&"0".repeat(BLOCKCHAIN_DIFFICULTY)) &&
            new_block.hash == new_block.create_hash()
    }

    pub fn add_block_to_chain(&mut self, new_block: &Block) -> bool {
        if !self.is_valid_new_block(new_block) {
            return false;
        }

        self.chain.push(new_block.clone());
        true
    }

    pub fn add_block_without_validation(&mut self, new_block: Block) {
        println!("> Block added by this node");
        self.chain.push(new_block);
    }


    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn create_genesis_block(&mut self, miner_address: String) -> Block {
        let mut genesis = Block::new(0, "0".to_string(), Vec::new(), miner_address);
        loop {
            if genesis.mine() {
                break;
            }
        }

        println!("Genesis block: {}", genesis);
        self.chain.push(genesis.clone());

        genesis
    }

    pub fn load_starting_block(&mut self, starting_block: Block) {
        self.chain.push(starting_block);
    }

    pub fn get_length(&self) -> usize {
        self.chain.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let blockchain = Blockchain::new();
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
        let blockchain = Blockchain::new();
        let prev_block = blockchain.get_latest_block();
        let mut new_block = Block::new(prev_block.index + 1, prev_block.hash.clone(), vec![], "miner_address".to_string());

        new_block.mine();

        let is_valid = blockchain.is_valid_new_block(&new_block);

        assert!(is_valid);
    }

    #[test]
    fn test_is_valid_new_block_invalid_prev_hash() {
        let blockchain = Blockchain::new();
        let prev_block = blockchain.get_latest_block();
        let mut new_block = Block::new(prev_block.index + 1, "invalidHash".to_string(), vec![], "miner_address".to_string());

        new_block.mine();

        let is_valid = blockchain.is_valid_new_block(&new_block);

        assert!(!is_valid);
    }

    #[test]
    fn test_is_valid_new_block_invalid_index() {
        let blockchain = Blockchain::new();
        let prev_block = blockchain.get_latest_block();
        let mut new_block = Block::new(prev_block.index + 10, prev_block.hash.clone(), vec![], "miner_address".to_string());

        new_block.mine();

        let is_valid = blockchain.is_valid_new_block(&new_block);

        assert!(!is_valid);
    }

    #[test]
    fn test_is_valid_new_block_invalid_hash() {
        let blockchain = Blockchain::new();
        let prev_block = blockchain.get_latest_block();
        let mut new_block = Block::new(prev_block.index + 1, prev_block.hash.clone(), vec![], "miner_address".to_string());

        new_block.mine();
        new_block.hash += "invalid";

        let is_valid = blockchain.is_valid_new_block(&new_block);

        assert!(!is_valid);
    }

    #[test]
    fn test_is_valid_new_block_invalid_hash_prefix() {
        let blockchain = Blockchain::new();
        let prev_block = blockchain.get_latest_block();
        let mut new_block = Block::new(prev_block.index + 1, prev_block.hash.clone(), vec![], "miner_address".to_string());

        new_block.mine();
        new_block.hash = new_block.hash[1..].to_string();

        let is_valid = blockchain.is_valid_new_block(&new_block);

        assert!(!is_valid);
    }

    #[test]
    fn test_get_latest_block() {
        let mut blockchain = Blockchain::new();
        let prev_block = blockchain.get_latest_block().clone();
        let new_block = Block::new(prev_block.index + 1, prev_block.hash.clone(), vec![], "miner_address".to_string());

        blockchain.add_block_to_chain(&new_block);

        let latest_block = blockchain.get_latest_block();

        assert_eq!(latest_block.index, new_block.index);
    }
}