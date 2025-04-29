use uuid::Uuid;
use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::wallet::Wallet;

#[derive(Clone)]
pub struct Node {
    pub name: String,
    pub blockchain: Blockchain,
    pub wallet: Wallet,
    pub mempool: Vec<Transaction>,
    pub id: Uuid
}

impl Node {
    pub fn new(name: &str, difficulty: usize) -> Self {
        Self {
            name: name.to_string(),
            blockchain: Blockchain::new(difficulty),
            wallet: Wallet::new(),
            mempool: Vec::new(),
            id: Uuid::new_v4()
        }
    }

    pub fn receive_block(&mut self, block: Block) -> bool {
        if self.blockchain.add_block_to_chain(block.clone()) {
            println!("{} accepted new block", self.name);
            true
        } else {
            println!("{} rejected the block", self.name);
            false
        }
    }

    pub fn create_transaction(&mut self, recipient: String, amount: u64) -> Transaction {
        let mut transaction = Transaction::new(self.wallet.get_public_key(), recipient, amount);
        transaction.signature = Some(self.wallet.create_signature(&transaction));

        self.mempool.push(transaction.clone());

        transaction
    }

    pub fn mine_block(&mut self) -> Option<Block> {
        let previous_hash = self.blockchain.get_latest_block().hash.clone();
        let mut block = Block::new(self.blockchain.get_chain().len() as u64, previous_hash, self.mempool.clone(), 3);

        self.mempool.clear();
        block.mine();

        self.blockchain.add_block_to_chain(block.clone());

        Some(block)
    }
}