use crate::models::FileMetadata;
use reqwest::Client;
use anyhow::Result;

const BACKEND_URL: &str = "http://127.0.0.1:8080";

pub async fn list_files() -> Result<Vec<FileMetadata>> {
    let client = Client::new();
    let resp = client.get(format!("{}/api/files", BACKEND_URL))
        .send()
        .await?
        .json()
        .await?;
    Ok(resp)
}

// We'll add upload later with progress
pub async fn upload_file(_file_path: String) -> Result<()> {
    // TODO: multipart upload to /api/upload
    Ok(())
}