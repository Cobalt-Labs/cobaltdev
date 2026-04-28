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
) -> impl IntoResponse {
    let config = match Config::load() {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };
    let storage = StorageService::new(config.storage_base_path.clone());

    let mut filename = String::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.file_name().unwrap_or("unknown").to_string();
        filename = name.clone();

        let data = match field.bytes().await {
            Ok(d) => d,
            Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        };

        let temp_path = format!("/tmp/{}", name);
        if let Err(e) = tokio::fs::write(&temp_path, &data).await {
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }

        let file = match tokio::fs::File::open(&temp_path).await {
            Ok(f) => f,
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        };

        let (storage_path, checksum, size_bytes) = match storage.upload_file(&claims.sub, &name, file).await {
            Ok(res) => res,
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        };
        
        let file_id = Uuid::new_v4().to_string();

        if let Err(e) = sqlx::query(
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
        .await {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)).into_response();
        }
        
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

pub async fn list_files_handler(
    State(pool): State<sqlx::SqlitePool>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let files: () = match sqlx::query_as::<_, crate::models::FileMetadata>(
        "SELECT id, filename, storage_path, owner_username, size_bytes, checksum, uploaded_at FROM files WHERE owner_username = ?"
    )
    .bind(&claims.sub)
    .fetch_all(&pool)
    .await {
        Ok(f) => f,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    (
        StatusCode::OK,
        [
            ("Access-Control-Allow-Origin", "*"),
            ("Access-Control-Allow-Methods", "GET, OPTIONS"),
            ("Access-Control-Allow-Headers", "Content-Type, Authorization"),
        ],
        Json(json!({
            "status": "success",
            "files": files
        }))
    ).into_response()
}