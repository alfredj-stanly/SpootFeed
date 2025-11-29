mod config;
mod routes;

use anyhow::Result;
use tracing::info;
use tracing_subscriber;

use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::from_env()?;
    let make_service = routes::app();

    let listener = tokio::net::TcpListener::bind(&config.server_addr()).await?;
    info!("Feeding on {}", &config.server_addr());

    axum::serve(listener, make_service).await?;

    Ok(())
}
