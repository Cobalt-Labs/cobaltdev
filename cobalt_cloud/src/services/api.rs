use crate::models::FileMetadata;
use reqwest::Client;
use anyhow::Result;

const BACKEND_URL: &str = "http://127.0.0.1:8080";

pub async fn list_files(token: Option<String>) -> Result<Vec<FileMetadata>> {
    let client = Client::new();
    let mut req = client.get(format!("{}/api/files", BACKEND_URL));

    if let Some(t) = token {
        req = req.bearer_auth(t);
    }

    let resp = req.send().await?.json().await?;
    Ok(resp)
}

// Placeholder - we'll make real multipart upload with progress later
pub async fn upload_file(_file_bytes: Vec<u8>, _filename: String, _token: Option<String>) -> Result<()> {
    // TODO: Implement real multipart with progress
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await; // simulate
    Ok(())
}