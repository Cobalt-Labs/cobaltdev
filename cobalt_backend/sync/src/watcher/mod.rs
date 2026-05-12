use crate::Config;
use crate::models::SyncEvent;
use anyhow::Result;
use notify::{RecursiveMode, Watcher};
use std::path::Path;
use tokio::sync::mpsc::Sender;
use tracing::{error, info};

pub async fn start_watcher(config: Config, tx: Sender<SyncEvent>) -> Result<()> {
    info!("Starting file watcher...");

    let (sync_tx, mut sync_rx) = tokio::sync::mpsc::unbounded_channel();

    let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        match res {
            Ok(event) => {
                let _ = sync_tx.send(event);
            }
            Err(e) => error!("Watcher error: {:?}", e),
        }
    })?;

    for path in &config.watch_paths {
        if path.exists() {
            watcher.watch(path, RecursiveMode::Recursive)?;
            info!("Watching: {:?}", path);
        } else {
            error!("Path does not exist: {:?}", path);
        }
    }

    while let Some(event) = sync_rx.recv().await {
        for path in event.paths {
            if event.kind.is_modify() {
                tx.send(SyncEvent::Modified(path)).await?;
            } else if event.kind.is_create() {
                tx.send(SyncEvent::Created(path)).await?;
            } else if event.kind.is_remove() {
                tx.send(SyncEvent::Deleted(path)).await?;
            }
        }
    }

    Ok(())
}
