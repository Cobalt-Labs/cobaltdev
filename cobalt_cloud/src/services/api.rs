use anyhow::Result;
use reqwest::multipart::{Form, Part};
use reqwest::{Client, Response};

const BACKEND_URL: &str = "http://127.0.0.1:8001";

pub async fn upload_file_bytes(filename: String, file_bytes: Vec<u8>, token: Option<String>) -> Result<()> {
    let client = Client::new();
    
    let part = Part::bytes(file_bytes).file_name(filename);
    let form = Form::new().part("file", part);

    let mut req = client
        .post(format!("{}/api/upload", BACKEND_URL))
        .multipart(form);
    
    if let Some(t) = token {
        req = req.bearer_auth(t);
    }

    let resp: Response = req.send().await?;

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

pub async fn signup(username: String, password: String) -> Result<()> {
    let client = Client::new();
    let resp = client
        .post(format!("{}/auth/register", BACKEND_URL))
        .json(&serde_json::json!({
            "username": username,
            "password": password,
        }))
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(anyhow::anyhow!("Signup failed: {}", resp.status()));
    }

    Ok(())
}

pub async fn forgot_password(username: String) -> Result<String> {
    let client = Client::new();
    let resp = client
        .post(format!("{}/auth/forgot-password", BACKEND_URL))
        .json(&serde_json::json!({
            "username": username,
        }))
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(anyhow::anyhow!("Request failed: {}", resp.status()));
    }

    let data: serde_json::Value = resp.json().await?;
    let message = data["message"].as_str().unwrap_or("Request sent").to_string();
    Ok(message)
}