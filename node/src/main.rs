use anyhow::Result;
use MockChain::init::init;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    init().await
}