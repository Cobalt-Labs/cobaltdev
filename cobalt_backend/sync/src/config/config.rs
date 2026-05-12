use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    pub watch_paths: Vec<PathBuf>,
    pub storage_backend: String,
    pub s3_bucket: String,
    pub s3_endpoint: String,
    pub s3_region: String,
    pub access_key: String,
    pub secret_key: String,
}

impl SyncConfig {
    pub fn load() -> Result<Self> {
        // In a real app, this would load from a file.
        // For now, we'll return a default/stub or try to load from environment.
        Ok(Self {
            watch_paths: vec![PathBuf::from("./test_sync")],
            storage_backend: "s3".to_string(),
            s3_bucket: "cobalt-sync".to_string(),
            s3_endpoint: "http://localhost:9000".to_string(),
            s3_region: "us-east-1".to_string(),
            access_key: "minioadmin".to_string(),
            secret_key: "minioadmin".to_string(),
        })
    }
}
