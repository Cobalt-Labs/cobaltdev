use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetadata {
    pub id: Uuid,
    pub filename: String,
    pub storage_path: String,
    pub owner_username: String,
    pub size_bytes: i64,
    pub checksum: String,
    pub uploaded_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct UploadResult {
    pub metadata: FileMetadata,
}