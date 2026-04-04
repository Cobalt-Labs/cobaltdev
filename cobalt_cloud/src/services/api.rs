use anyhow::Result;
use reqwest::multipart::{Form, Part};
use reqwest::{Client, Response}; // Explicitly import Response

const BACKEND_URL: &str = "http://127.0.0.1:8080";

#[cfg(not(target_arch = "wasm32"))]
pub async fn upload_file(file_path: String) -> Result<()> {
    let client = Client::new();
    let file_bytes = tokio::fs::read(&file_path).await?;

    let filename = std::path::Path::new(&file_path)
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?
        .to_string_lossy()
        .to_string();

    let part = Part::bytes(file_bytes).file_name(filename);
    let form = Form::new().part("file", part);

    // Added explicit type for Response to satisfy the compiler
    let resp: Response = client
        .post(format!("{}/api/upload", BACKEND_URL))
        .multipart(form)
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(anyhow::anyhow!("Upload failed: {}", resp.status()));
    }

    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub async fn upload_file(_file_path: String) -> Result<()> {
    Err(anyhow::anyhow!("Web upload coming soon. Use desktop version for now."))
}

pub async fn list_files(token: Option<String>) -> Result<Vec<crate::models::FileMetadata>> {
    let client = Client::new();
    let mut req = client.get(format!("{}/api/files", BACKEND_URL));

    if let Some(t) = token {
        req = req.bearer_auth(t);
    }

    // FIX 1: Type annotation for the request send
    let resp: Response = req.send().await?;
    
    // FIX 2: Explicitly turbofish the JSON type to avoid "multiple candidates" 
    // and help the compiler distinguish your model.
    let data = resp.json::<Vec<crate::models::FileMetadata>>().await?;
    
    Ok(data)
}