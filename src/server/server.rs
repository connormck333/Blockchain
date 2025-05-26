use std::net::SocketAddr;
use axum::{Json, Router};
use axum::routing::post;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Serialize, Deserialize, Debug)]
struct TransactionData {
    sender: String,
    recipient: String,
    amount: u64
}

pub async fn start_server() -> anyhow::Result<()> {
    let app = Router::new().route("/transaction", post(handle_transaction));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("Server started, listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn handle_transaction(Json(payload): Json<TransactionData>) -> String {
    "Received".to_string()
}