use anyhow::Result;
use reqwest::multipart::{Form, Part};
use reqwest::{Client, Response};

const BACKEND_URL: &str = "http://127.0.0.1:8080";

pub async fn upload_file_bytes(filename: String, file_bytes: Vec<u8>) -> Result<()> {
    let client = Client::new();
    
    let part = Part::bytes(file_bytes).file_name(filename);
    let form = Form::new().part("file", part);

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