use crate::chain::block::Block;
use crate::chain::block_validation_type::BlockValidationType;
use crate::constants::BLOCKCHAIN_DIFFICULTY;

#[derive(Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub invalid_blocks: Vec<Block>,
    pub pending_blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            chain: vec![],
            invalid_blocks: vec![],
            pending_blocks: vec![],
        }
    }

    pub fn is_valid_new_block(&mut self, new_block: &Block) -> BlockValidationType {
        if self.chain.len() == 0 {
            return BlockValidationType::Valid;
        }

        let last_block = self.chain.last().unwrap();
        println!("Checking new block validity, current length {}", self.chain.len());

        if new_block.previous_block_hash != last_block.hash {
            println!("Found invalid previous block hash");
            println!("Storing block in case of forked chain");
            self.invalid_blocks.push(new_block.clone());

            return BlockValidationType::Fork;
        }

        if new_block.index == last_block.index + 1 && new_block.hash.starts_with(&"0".repeat(BLOCKCHAIN_DIFFICULTY)) && new_block.hash == new_block.create_hash() {
            return BlockValidationType::Valid;
        }

        BlockValidationType::Invalid
    }

    pub fn add_pending_block(&mut self, new_block: Block) {
        self.pending_blocks.push(new_block);
    }

    pub fn add_block_to_chain(&mut self, new_block: &Block) -> BlockValidationType {
        let block_validation_type = self.is_valid_new_block(new_block);
        if block_validation_type == BlockValidationType::Valid {
            self.chain.push(new_block.clone());
        }

        block_validation_type
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
        let mut blockchain = Blockchain::new();
        let prev_block = blockchain.get_latest_block();
        let mut new_block = Block::new(prev_block.index + 1, prev_block.hash.clone(), vec![], "miner_address".to_string());

        new_block.mine();

        let is_valid = blockchain.is_valid_new_block(&new_block);

        assert_eq!(is_valid, BlockValidationType::Valid);
    }

    #[test]
    fn test_is_valid_new_block_invalid_prev_hash() {
        let mut blockchain = Blockchain::new();
        let prev_block = blockchain.get_latest_block();
        let mut new_block = Block::new(prev_block.index + 1, "invalidHash".to_string(), vec![], "miner_address".to_string());

        new_block.mine();

        let is_valid = blockchain.is_valid_new_block(&new_block);

        assert_eq!(is_valid, BlockValidationType::Invalid);
    }

    #[test]
    fn test_is_valid_new_block_invalid_index() {
        let mut blockchain = Blockchain::new();
        let prev_block = blockchain.get_latest_block();
        let mut new_block = Block::new(prev_block.index + 10, prev_block.hash.clone(), vec![], "miner_address".to_string());

        new_block.mine();

        let is_valid = blockchain.is_valid_new_block(&new_block);

        assert_eq!(is_valid, BlockValidationType::Invalid);
    }

    #[test]
    fn test_is_valid_new_block_invalid_hash() {
        let mut blockchain = Blockchain::new();
        let prev_block = blockchain.get_latest_block();
        let mut new_block = Block::new(prev_block.index + 1, prev_block.hash.clone(), vec![], "miner_address".to_string());

        new_block.mine();
        new_block.hash += "invalid";

        let is_valid = blockchain.is_valid_new_block(&new_block);

        assert_eq!(is_valid, BlockValidationType::Invalid);
    }

    #[test]
    fn test_is_valid_new_block_invalid_hash_prefix() {
        let mut blockchain = Blockchain::new();
        let prev_block = blockchain.get_latest_block();
        let mut new_block = Block::new(prev_block.index + 1, prev_block.hash.clone(), vec![], "miner_address".to_string());

        new_block.mine();
        new_block.hash = new_block.hash[1..].to_string();

        let is_valid = blockchain.is_valid_new_block(&new_block);

        assert_eq!(is_valid, BlockValidationType::Invalid);
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