use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FileMetadata {
    pub id: Uuid,
    pub filename: String,
    pub storage_path: String,
    pub owner_username: String,
    pub size_bytes: i64,
    pub checksum: String,
    pub encrypted_checksum: String,
    pub uploaded_at: DateTime<Utc>,
    pub is_encrypted: bool,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub username: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

impl Claims {
    pub fn new(username: String) -> Self {
        let exp = (Utc::now() + chrono::Duration::hours(24)).timestamp() as usize;
        Self { sub: username, exp }
    }
}