use crate::block::Block;
use crate::transaction::Transaction;

/*
    Blockchain is a shared, immutable digital ledger, enabling the recording of transactions
    and the tracking of assets within a business network and providing a single source of truth.
*/

pub struct Blockchain {
    chain: Vec<Block>,
    mempool: Vec<Transaction>,
    difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let genesis_block = Self::create_genesis_block(difficulty);
        Self {
            chain: vec![genesis_block],
            mempool: vec![],
            difficulty
        }
    }

    pub fn is_valid_new_block(&self, new_block: &Block) -> bool {
        let last_block = self.chain.last().unwrap();

        new_block.previous_block_hash == last_block.hash &&
        new_block.index == last_block.index + 1 &&
        new_block.hash.starts_with(&"0".repeat(self.difficulty)) &&
        new_block.hash == new_block.create_hash()
    }

    pub fn add_block_to_chain(&mut self, new_block: Block) {
        for transaction in new_block.transactions.iter() {
            let index = self.mempool.iter().position(|t| t.id == transaction.id);

            match index {
                Some(i) => {
                    self.mempool.remove(i);
                },
                None => {}
            }
        }

        self.chain.push(new_block);
    }

    pub fn get_chain(&self) -> &Vec<Block> {
        &self.chain
    }

    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn add_transaction_to_mempool(&mut self, transaction: Transaction) {
        self.mempool.push(transaction);
    }

    pub fn get_mempool(&self) -> &Vec<Transaction> {
        &self.mempool
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
        assert!(blockchain.mempool.is_empty());

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
    fn test_add_block_to_chain_empty_mempool() {
        let mut blockchain = Blockchain::new(2);
        let prev_block = blockchain.get_latest_block();
        let new_block = Block::new(prev_block.index + 1, prev_block.hash.clone(), vec![], 2);

        blockchain.add_block_to_chain(new_block.clone());

        let added_block = blockchain.get_latest_block();

        assert!(blockchain.mempool.is_empty());
        assert_eq!(blockchain.chain.len(), 2);
        assert_eq!(added_block.index, new_block.index);
    }

    #[test]
    fn test_add_block_to_chain_populated_mempool() {
        let mut blockchain = Blockchain::new(2);
        let prev_block = blockchain.get_latest_block().clone();

        blockchain.add_transaction_to_mempool(create_transaction("id1".to_string()));
        blockchain.add_transaction_to_mempool(create_transaction("id2".to_string()));
        blockchain.add_transaction_to_mempool(create_transaction("id3".to_string()));

        let new_block = Block::new(prev_block.index + 1, prev_block.hash.clone(), blockchain.mempool.clone(), 2);

        blockchain.add_block_to_chain(new_block.clone());

        let added_block = blockchain.get_latest_block();

        assert!(blockchain.mempool.is_empty());
        assert_eq!(blockchain.chain.len(), 2);
        assert_eq!(added_block.index, new_block.index);
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

    #[test]
    fn test_add_transaction_to_mempool() {
        let mut blockchain = Blockchain::new(2);

        blockchain.add_transaction_to_mempool(create_transaction("id".to_string()));

        assert_eq!(blockchain.mempool.len(), 1);
        assert_eq!(blockchain.mempool[0].id, "id");
    }

    #[test]
    fn test_get_mempool() {
        let mut blockchain = Blockchain::new(2);

        blockchain.add_transaction_to_mempool(create_transaction("id1".to_string()));
        blockchain.add_transaction_to_mempool(create_transaction("id2".to_string()));

        let mempool = blockchain.get_mempool();

        assert_eq!(mempool.len(), 2);
        assert_eq!(mempool[0].id, "id1");
        assert_eq!(mempool[1].id, "id2");
    }

    fn create_transaction(id: String) -> Transaction {
        Transaction::new("sender".to_string(), "recipient".to_string(), 100)
    }
}