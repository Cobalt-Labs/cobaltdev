use axum::{
    extract::{State, Multipart},
    Json,
    http::StatusCode,
};
use serde_json::json;
use crate::config::config::Config;
use crate::services::storage::StorageService;

pub async fn upload_file_handler(
    State(pool): State<sqlx::SqlitePool>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let config = Config::load().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let storage = StorageService::new(config.storage_base_path.clone());

    let mut filename = String::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))? {
        let name = field.file_name().unwrap_or("unknown").to_string();
        filename = name.clone();

        let data = field.bytes().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let temp_path = format!("/tmp/{}", name);
        tokio::fs::write(&temp_path, &data).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let file = tokio::fs::File::open(&temp_path).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let (storage_path, checksum, _) = storage.upload_file("ibrahim3595", &name, file).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        sqlx::query(
            "INSERT INTO files (user_id, filename, storage_path, checksum, created_at) VALUES (?, ?, ?, ?, ?)"
        )
        .bind("ibrahim3595")
        .bind(&name)
        .bind(&storage_path)
        .bind(&checksum)
        .bind(chrono::Utc::now())
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)))?;
        
        println!("Secure upload: {} → {}", name, storage_path);
    }

    Ok(Json(json!({
        "status": "success",
        "filename": filename,
        "message": "File uploaded and saved securely"
    })))
}

pub async fn list_files_handler() -> Json<serde_json::Value> {
    Json(json!({"status": "ok", "files": []}))
}