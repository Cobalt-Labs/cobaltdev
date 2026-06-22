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
        // Resolve the workspace root relative to this binary's manifest directory.
        // Falls back to an absolute path pointing at cobalt_backend/cloud/storage.
        let workspace_root = std::env::var("COBALT_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                // Walk up from the binary location to find the workspace root
                // (3 levels: target/debug/<bin> → workspace root)
                std::env::current_exe()
                    .ok()
                    .and_then(|p| {
                        // target/debug → target → workspace root
                        p.ancestors().nth(3).map(|a| a.to_path_buf())
                    })
                    .unwrap_or_else(|| PathBuf::from("/Users/ibrahimhaji/code/cobaltdev"))
            });

        let storage_path = workspace_root.join("cobalt_backend/cloud/storage");

        Ok(Self {
            watch_paths: vec![storage_path],
            storage_backend: "s3".to_string(),
            s3_bucket: "cobalt-sync".to_string(),
            s3_endpoint: "http://localhost:9000".to_string(),
            s3_region: "us-east-1".to_string(),
            access_key: "minioadmin".to_string(),
            secret_key: "minioadmin".to_string(),
        })
    }
}
