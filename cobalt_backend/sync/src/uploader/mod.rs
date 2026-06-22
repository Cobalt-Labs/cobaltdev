use crate::Config;
use crate::models::SyncEvent;
use anyhow::Result;
use opendal::Operator;
use opendal::services::S3;
use tokio::sync::mpsc::Receiver;
use tracing::{error, info};

pub async fn start_uploader(config: Config, mut rx: Receiver<SyncEvent>) -> Result<()> {
    info!("Starting uploader for bucket: {}", config.s3_bucket);

    let builder = S3::default()
        .bucket(&config.s3_bucket)
        .endpoint(&config.s3_endpoint)
        .region(&config.s3_region)
        .access_key_id(&config.access_key)
        .secret_access_key(&config.secret_key);

    let op: Operator = Operator::new(builder)?.finish();

    while let Some(event) = rx.recv().await {
        match event {
            SyncEvent::Created(path) | SyncEvent::Modified(path) => {
                if path.is_file() {
                    let file_name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");
                    info!("Uploading file: {}", file_name);

                    match tokio::fs::read(&path).await {
                        Ok(content) => {
                            if let Err(e) = op.write(file_name, content).await {
                                error!("Failed to upload {}: {:?}", file_name, e);
                            } else {
                                info!("Successfully uploaded {}", file_name);
                            }
                        }
                        Err(e) => error!("Failed to read file {:?}: {:?}", path, e),
                    }
                }
            }
            SyncEvent::Deleted(path) => {
                let file_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                info!("Deleting file from S3: {}", file_name);
                if let Err(e) = op.delete(file_name).await {
                    error!("Failed to delete {}: {:?}", file_name, e);
                }
            }
        }
    }

    Ok(())
}
