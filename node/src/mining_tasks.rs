use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Mutex;
use crate::block::Block;
use crate::constants::{MINING_REWARD_AMOUNT, MINING_REWARD_DELAY};
use crate::database::operations::DbOperations;
use crate::mining_reward::MiningReward;
use crate::network::message::Message;
use crate::network::message_sender::broadcast_message;
use crate::network::node::Node;
use crate::transaction::Transaction;
use crate::wallet::Wallet;

pub fn spawn_mining_loop(
    node: Arc<Mutex<Node>>,
    mining_flag: Arc<AtomicBool>,
    db: DbOperations
) {
    tokio::spawn(async move {
        loop {
            if !mining_flag.load(Ordering::Relaxed) {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                continue;
            }
            let mined_block: Option<Block> = spawn_mining(node.clone(), mining_flag.clone()).await;

            if let Some(block) = mined_block {
                node.lock().await.blockchain.add_block_without_validation(block.clone());
                node.lock().await.delete_txs_from_mempool(&block.transactions).await;

                let node_address = node.lock().await.wallet.address.clone();
                save_mining_reward(db.clone(), node_address, block.index).await;

                let transactions = block.transactions.clone();
                let mined_block_message = Message::BlockMined {
                    from: node.lock().await.address.clone(),
                    block
                };
                broadcast_message(node.clone(), &mined_block_message).await;

                spawn_update_balances(db.clone(), transactions);
            } else {
                if node.lock().await.blockchain.invalid_blocks.len() < 5 {
                    println!("Resuming mining...");
                    mining_flag.store(true, Ordering::Relaxed);
                }
            }
        }
    });
}

async fn spawn_mining(node: Arc<Mutex<Node>>, mining_flag: Arc<AtomicBool>) -> Option<Block> {
    tokio::task::spawn_blocking({
        let cancel_flag = mining_flag.clone();
        let node_inner = node.clone();
        let transactions = node_inner.lock().await.mempool.lock().await.clone();
        let blockchain_clone = node_inner.lock().await.blockchain.clone();
        let node_address = node_inner.lock().await.wallet.address.clone();
        move || mine_block(
            transactions,
            blockchain_clone.get_latest_block().clone().hash,
            blockchain_clone.get_length() as u64,
            cancel_flag,
            node_address
        )
    }).await.unwrap()
}

fn mine_block(
    transactions: Vec<Transaction>,
    previous_hash: String,
    block_index: u64,
    cancel_flag: Arc<AtomicBool>,
    node_address: String
) -> Option<Block> {
    // println!("-----> Transaction count picked up for mining: {}", transactions.len());
    let mut block = Block::new(block_index, previous_hash, transactions, node_address);

    while cancel_flag.load(Ordering::Relaxed) == true {
        if block.mine() {
            println!("Mined block {}", block.index);
            return Some(block);
        }
    }

    None
}

async fn save_mining_reward(db: DbOperations, node_address: String, block_index: u64) {
    let mining_reward = MiningReward::new(
        MINING_REWARD_AMOUNT,
        node_address,
        block_index + MINING_REWARD_DELAY
    );
    db.save_mining_reward(mining_reward).await;
}

pub fn spawn_update_balances(db: DbOperations, transactions: Vec<Transaction>) {
    println!("Started update balances");
    tokio::spawn(async move {
        println!("Mined transactions count: {}", transactions.len());
        for transaction in &transactions {
            // Decrement sender balance
            let sender_address = Wallet::derive_address_hash_from_string(&transaction.sender);
            db.create_user_and_update_balance(sender_address.clone(), -(transaction.amount as i64)).await;
            println!("Updated balance for {}", sender_address);

            // Increment recipient balance
            let receiver_address = transaction.recipient.clone();
            db.create_user_and_update_balance(receiver_address.clone(), transaction.amount as i64).await;
            println!("Updated balance for {}", receiver_address);
        }
    });
}