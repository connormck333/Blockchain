use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Json, Router};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use tokio::net::TcpListener;
use crate::database::validator::Validator;
use crate::node::Mempool;
use crate::server::request::transaction::TransactionRequest;
use crate::chain::transaction::Transaction;
use crate::chain::wallet::Wallet;
use crate::server::response::transaction_response::TransactionResponse;

#[derive(Clone)]
struct ServerState {
    mempool: Mempool,
    validator: Arc<Validator>
}

pub async fn start_server(mempool: Mempool, validator: Arc<Validator>) -> anyhow::Result<()> {
    let state = ServerState { mempool, validator };
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
    Json(payload): Json<TransactionRequest>
) -> impl IntoResponse {
    let user_wallet = Wallet::load_from_public_key(payload.sender_public_key.clone());

    let transaction = Transaction::load(payload);
    if !user_wallet.verify_signature(&transaction) {
        let response = TransactionResponse::new(false, "Invalid signature".to_string());
        return (StatusCode::BAD_REQUEST, Json(response))
    }

    if !state.validator.validate_transaction(&transaction).await {
        let response = TransactionResponse::new(false, "Insufficient funds".to_string());
        return (StatusCode::BAD_REQUEST, Json(response))
    }

    state.mempool.lock().await.push(transaction);

    println!("Transaction added to mempool.");

    let response = TransactionResponse::new(true, "Transaction added to mempool".to_string());
    (StatusCode::OK, Json(response))
}