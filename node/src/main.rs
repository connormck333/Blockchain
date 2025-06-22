use anyhow::Result;
use MockChain::init::init;

#[tokio::main]
async fn main() -> Result<()> {
    init().await
}