use serde::{Deserialize, Serialize};

/// Matches the backend's FileMetadata model exactly.
/// Backend stores id as TEXT and uploaded_at as TEXT (rfc3339 string).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FileMetadata {
    pub id: String,
    pub filename: String,
    pub storage_path: String,
    pub owner_username: String,
    pub size_bytes: i64,
    pub checksum: String,
    pub uploaded_at: String,
}