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

    pub fn create_new_block(&mut self, transactions: Vec<Transaction>) {
        let latest_block = self.chain.last().unwrap();
        let previous_block_hash = latest_block.previous_block_hash.clone();
        let index: u64 = self.chain.len() as u64;
        let new_block = Block::new(index, previous_block_hash, transactions, self.difficulty);

        self.chain.push(new_block);
    }

    pub fn is_valid_new_block(&self, new_block: &Block) -> bool {
        let last_block = self.chain.last().unwrap();

        new_block.previous_block_hash == last_block.hash &&
        new_block.index == last_block.index + 1 &&
        new_block.hash == new_block.create_hash() &&
        new_block.hash.starts_with(&"0".repeat(self.difficulty))
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

    pub fn get_difficulty(&self) -> usize {
        self.difficulty
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