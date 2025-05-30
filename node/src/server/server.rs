use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Json, Router};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use tokio::net::TcpListener;
use crate::database::validator::Validator;
use crate::network::node::Mempool;
use crate::server::request::transaction::TransactionRequest;
use crate::server::response::create_user::CreateUserResponse;
use crate::transaction::Transaction;
use crate::wallet::Wallet;

#[derive(Clone)]
struct ServerState {
    mempool: Mempool,
    validator: Arc<Validator>
}

pub async fn start_server(mempool: Mempool, validator: Arc<Validator>) -> anyhow::Result<()> {
    let state = ServerState { mempool, validator };
    let app = Router::new()
        .route("/create-user", post(create_user))
        .route("/transaction", post(handle_transaction))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("Server started, listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn create_user(
    State(state): State<ServerState>
) -> impl IntoResponse {
    let new_wallet = Wallet::new();

    if state.validator.db_connection.create_user(&new_wallet).await {
        return (
            StatusCode::OK,
            Json(CreateUserResponse::new(new_wallet))
        ).into_response()
    }

    (StatusCode::INTERNAL_SERVER_ERROR, Json("There was an error.")).into_response()
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