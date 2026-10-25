use anyhow::Result;
use cobalt_sync::Config;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("Cobalt Sync Daemon starting...");

    let config = Config::load()?;

    tracing::info!("Watching paths: {:?}", config.watch_paths);
    tracing::info!("Backend: {}", config.storage_backend);

    // Start the sync process
    cobalt_sync::start_sync(config).await?;

    tracing::info!("Cobalt Sync has finished.");
    
    Ok(())
}