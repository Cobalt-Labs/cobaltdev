use anyhow::Result;
use reqwest::multipart::{Form, Part};
use reqwest::{Client, Response};

const BACKEND_URL: &str = "http://localhost:8001";

pub async fn upload_file_bytes(filename: String, file_bytes: Vec<u8>, token: Option<String>) -> Result<()> {
    let client = Client::new();
    let full_url = format!("{}/api/upload", BACKEND_URL);

    let part = Part::bytes(file_bytes)
        .file_name(filename.clone())
        .mime_str("application/octet-stream")
        .map_err(|e| anyhow::anyhow!("Mime error: {}", e))?;

    let form = Form::new().part("file", part);

    let mut req = client.post(&full_url).multipart(form);

    if let Some(t) = token {
        req = req.bearer_auth(t);
    }

    let resp: Response = req.send().await
        .map_err(|e| anyhow::anyhow!("Upload connection error: {}. (File: {}, Target: {})", e, filename, full_url))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let err_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(anyhow::anyhow!("Upload failed ({}): {}", status, err_text));
    }

    Ok(())
}

pub async fn list_files(token: Option<String>) -> Result<Vec<crate::models::FileMetadata>> {
    let client = Client::new();
    let mut req = client.get(format!("{}/api/files", BACKEND_URL));

    if let Some(t) = token {
        req = req.bearer_auth(t);
    }

    let resp: Response = req.send().await
        .map_err(|e| anyhow::anyhow!("Connection error: {}. (URL: {}/api/files)", e, BACKEND_URL))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let err_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(anyhow::anyhow!("Fetch failed ({}): {}", status, err_text));
    }

    let data: serde_json::Value = resp.json().await?;
    let files = serde_json::from_value::<Vec<crate::models::FileMetadata>>(
        data["files"].clone()
    ).map_err(|e| anyhow::anyhow!("JSON parse error: {}. Raw: {}", e, data))?;

    Ok(files)
}

pub async fn login(username: String, password: String) -> Result<crate::hooks::use_auth::AuthState> {
    let u = username.trim().to_string();
    let p = password.trim().to_string();

    if u.is_empty() || p.is_empty() {
        return Err(anyhow::anyhow!("Username and password cannot be empty"));
    }

    let client = Client::new();
    let resp = client
        .post(format!("{}/auth/login", BACKEND_URL))
        .json(&serde_json::json!({
            "username": u,
            "password": p,
        }))
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Cannot connect to server: {}. Is the backend running?", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let err_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(anyhow::anyhow!("Login failed ({}): {}", status, err_text));
    }

    // Backend returns { token: "...", username: "..." }
    let data: serde_json::Value = resp.json().await?;
    let token = data["token"].as_str()
        .ok_or_else(|| anyhow::anyhow!("No token in response"))?
        .to_string();
    let username_resp = data["username"].as_str().unwrap_or(&u).to_string();

    Ok(crate::hooks::use_auth::AuthState {
        token: Some(token),
        username: Some(username_resp),
    })
}

pub async fn signup(username: String, password: String) -> Result<()> {
    let u = username.trim().to_string();
    let p = password.trim().to_string();

    if u.is_empty() || p.is_empty() {
        return Err(anyhow::anyhow!("Username and password cannot be empty"));
    }

    let client = Client::new();
    let resp = client
        .post(format!("{}/auth/register", BACKEND_URL))
        .json(&serde_json::json!({
            "username": u,
            "password": p,
        }))
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Cannot connect to server: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let err_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(anyhow::anyhow!("Signup failed ({}): {}", status, err_text));
    }

    Ok(())
}

pub async fn forgot_password(username: String) -> Result<String> {
    let u = username.trim().to_string();

    if u.is_empty() {
        return Err(anyhow::anyhow!("Username cannot be empty"));
    }

    let client = Client::new();
    let resp = client
        .post(format!("{}/auth/forgot-password", BACKEND_URL))
        .json(&serde_json::json!({ "username": u }))
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Cannot connect to server: {}", e))?;

    if !resp.status().is_success() {
        return Err(anyhow::anyhow!("Request failed: {}", resp.status()));
    }

    let data: serde_json::Value = resp.json().await?;
    let message = data["message"].as_str().unwrap_or("Request sent").to_string();
    Ok(message)
}