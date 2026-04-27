use axum::{
    extract::{State, Multipart},
    Json,
    http::StatusCode,
    Extension,
    response::IntoResponse,
};
use uuid::Uuid;
use crate::models::Claims;
use serde_json::json;
use crate::config::config::Config;
use crate::services::storage::StorageService;

pub async fn upload_file_handler(
    State(pool): State<sqlx::SqlitePool>,
    Extension(claims): Extension<Claims>,
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

        let (storage_path, checksum, size_bytes) = storage.upload_file(&claims.sub, &name, file).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        let file_id = Uuid::new_v4().to_string();

        sqlx::query(
            "INSERT INTO files (id, filename, storage_path, owner_username, size_bytes, checksum, uploaded_at) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&file_id)
        .bind(&name)
        .bind(&storage_path)
        .bind(&claims.sub)
        .bind(size_bytes)
        .bind(&checksum)
        .bind(chrono::Utc::now().to_rfc3339())
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)))?;
        
        println!("Secure upload: {} → {}", name, storage_path);
    }

    let res = json!({
        "status": "success",
        "filename": filename,
        "message": "File uploaded and saved securely"
    });

    (
        StatusCode::OK,
        [
            ("Access-Control-Allow-Origin", "*"),
            ("Access-Control-Allow-Methods", "POST, OPTIONS"),
            ("Access-Control-Allow-Headers", "Content-Type, Authorization"),
        ],
        Json(res)
    ).into_response()
}

pub async fn list_files_handler() -> Json<serde_json::Value> {
    Json(json!({"status": "ok", "files": []}))
}