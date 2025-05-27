use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Json, Router};
use axum::extract::State;
use axum::routing::post;
use tokio::net::TcpListener;
use crate::database::connection::Connection;
use crate::network::node::Mempool;
use crate::server::request_bodies::{CreateUserData, TransactionData};
use crate::wallet::Wallet;

#[derive(Clone)]
struct ServerState {
    mempool: Mempool,
    database: Arc<Connection>
}

pub async fn start_server(mempool: Mempool, database: Arc<Connection>) -> anyhow::Result<()> {
    let state = ServerState { mempool, database };
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
    State(state): State<ServerState>,
    Json(payload): Json<CreateUserData>
) -> String {
    assert!(payload.username.len() > 5);
    assert!(payload.password.len() > 5);

    let new_wallet = Wallet::new();

    let db_response = state.database.create_user(
        payload.username.as_str(),
        payload.password.as_str(),
        new_wallet
    ).await;

    if db_response.is_err() {
        return "Error creating user".to_string();
    }

    "User Created".to_string()
}

async fn handle_transaction(
    State(state): State<ServerState>,
    Json(payload): Json<TransactionData>
) -> String {
    // let public_key = state.wallet.lock().await.get_public_key();
    // let mut transaction = Transaction::new(public_key, payload.recipient, payload.amount);
    // transaction.signature = Some(state.wallet.lock().await.create_signature(&transaction));
    //
    // state.mempool.lock().await.push(transaction);
    //
    // println!("Transaction added to mempool.");

    "Received".to_string()
}