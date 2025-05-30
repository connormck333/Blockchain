use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Json, Router};
use axum::extract::State;
use axum::routing::post;
use tokio::net::TcpListener;
use crate::database::validator::Validator;
use crate::network::node::Mempool;
use crate::server::request::transaction::TransactionRequest;
use crate::transaction::Transaction;

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
) -> String {
    let db_response = state.validator.db_connection.get_user(
        payload.sender_public_key.clone()
    ).await;

    let user_wallet = match db_response {
        Ok(user) => user,
        Err(e) => return e.to_string()
    };

    let transaction = Transaction::load(payload);
    if !user_wallet.verify_signature(&transaction) {
        return "Invalid signature".to_string();
    }

    if !state.validator.validate_transaction(&transaction).await {
        return "Insufficient funds".to_string();
    }

    state.mempool.lock().await.push(transaction);

    println!("Transaction added to mempool.");

    "Received".to_string()
}