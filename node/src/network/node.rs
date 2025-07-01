use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::block::Block;
use crate::block_validation_type::BlockValidationType;
use crate::blockchain::Blockchain;
use crate::network::message::ChainLength;
use crate::network::peer::Peer;
use crate::transaction::Transaction;
use crate::wallet::Wallet;

pub type Mempool = Arc<Mutex<Vec<Transaction>>>;

pub struct Node {
    pub blockchain: Blockchain,
    pub mempool: Mempool,
    pub wallet: Wallet,
    pub id: Uuid,
    pub address: String,
    pub peers: HashMap<String, Peer>,
    pub max_peer_chain_length: Option<ChainLength>
}

impl Node {
    pub fn new(address: String) -> Self {
        Self {
            blockchain: Blockchain::new(),
            mempool: Arc::new(Mutex::new(Vec::new())),
            wallet: Wallet::new(),
            id: Uuid::new_v4(),
            address,
            peers: HashMap::new(),
            max_peer_chain_length: None
        }
    }

    pub fn receive_block(&mut self, block: &Block) -> BlockValidationType {
        self.blockchain.add_block_to_chain(&block)
    }

    pub async fn delete_txs_from_mempool(&mut self, transactions: &Vec<Transaction>) {
        self.mempool.lock().await.retain(|tx| !transactions.contains(tx));
    }

    pub fn add_peer(&mut self, address: String, writer: OwnedWriteHalf, reader: OwnedReadHalf) {
        self.peers.insert(address.clone(), Peer::new(address, writer, reader));
    }

    pub fn get_peer(&mut self, address: &str) -> Option<&mut Peer> {
        self.peers.get_mut(address)
    }
}