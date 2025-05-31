use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::wallet::Wallet;

pub type Mempool = Arc<Mutex<Vec<Transaction>>>;

#[derive(Clone)]
pub struct Node {
    pub name: String,
    pub blockchain: Blockchain,
    pub mempool: Mempool,
    pub wallet: Wallet,
    pub id: Uuid,
    pub difficulty: usize
}

impl Node {
    pub fn new(name: &str, difficulty: usize) -> Self {
        Self {
            name: name.to_string(),
            blockchain: Blockchain::new(difficulty),
            mempool: Arc::new(Mutex::new(Vec::new())),
            wallet: Wallet::new(),
            id: Uuid::new_v4(),
            difficulty
        }
    }

    pub fn receive_block(&mut self, block: &Block) -> bool {
        if self.blockchain.add_block_to_chain(&block) {
            println!("{} accepted new block", self.name);
            true
        } else {
            println!("{} rejected the block", self.name);
            false
        }
    }

    pub async fn delete_txs_from_mempool(&mut self, transactions: &Vec<Transaction>) {
        self.mempool.lock().await.retain(|tx| !transactions.contains(tx));
    }
}