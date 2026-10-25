pub mod config;
pub mod models;
pub mod uploader;
pub mod watcher;

pub use config::config::SyncConfig as Config;
pub use models::models::SyncEvent;

pub async fn start_sync(config: Config) -> anyhow::Result<()> {
    let (tx, rx) = tokio::sync::mpsc::channel(100);
    
    let watcher_config = config.clone();
    let watcher_handle = tokio::spawn(async move {
        watcher::start_watcher(watcher_config, tx).await
    });

    let uploader_config = config.clone();
    let uploader_handle = tokio::spawn(async move {
        uploader::start_uploader(uploader_config, rx).await
    });

    // Wait for either to fail or for shutdown (managed in main.rs)
    tokio::select! {
        res = watcher_handle => res??,
        res = uploader_handle => res??,
    }

    Ok(())
}