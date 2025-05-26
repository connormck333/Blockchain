use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Json, Router};
use axum::extract::State;
use axum::routing::post;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use crate::network::node::Mempool;
use crate::transaction::Transaction;
use crate::wallet::Wallet;

#[derive(Serialize, Deserialize, Debug)]
struct TransactionData {
    recipient: String,
    amount: u64
}

#[derive(Clone)]
struct ServerState {
    mempool: Mempool,
    wallet: Arc<Mutex<Wallet>>
}

pub async fn start_server(mempool: Mempool, wallet: Arc<Mutex<Wallet>>) -> anyhow::Result<()> {
    let state = ServerState { mempool, wallet };
    let app = Router::new()
        .route("/transaction", post(handle_transaction))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("Server started, listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn handle_transaction(
    State(state): State<ServerState>,
    Json(payload): Json<TransactionData>
) -> String {
    let public_key = state.wallet.lock().await.get_public_key();
    let mut transaction = Transaction::new(public_key, payload.recipient, payload.amount);
    transaction.signature = Some(state.wallet.lock().await.create_signature(&transaction));

    state.mempool.lock().await.push(transaction);

    println!("Transaction added to mempool.");

    "Received".to_string()
}