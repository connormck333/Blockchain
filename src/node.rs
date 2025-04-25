use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::wallet::Wallet;

#[derive(Clone)]
pub struct Node {
    pub name: String,
    pub blockchain: Blockchain,
    pub wallet: Wallet
}

impl Node {
    pub fn new(name: &str, difficulty: usize) -> Self {
        Self {
            name: name.to_string(),
            blockchain: Blockchain::new(difficulty),
            wallet: Wallet::new()
        }
    }

    pub fn receive_block(&mut self, block: Block) -> bool {
        if self.blockchain.is_valid_new_block(&block) {
            self.blockchain.add_block_to_chain(block.clone());
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

        self.blockchain.add_transaction_to_mempool(transaction.clone());

        transaction
    }
}