use crate::config::config::Config;
use crate::models::{Claims, FileMetadata};
use crate::services::storage::StorageService;
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde_json::json;
use uuid::Uuid;

pub async fn upload_file_handler(
    State(pool): State<sqlx::SqlitePool>,
    Extension(claims): Extension<Claims>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let config = match Config::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Config load error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Server configuration error"})),
            )
                .into_response();
        }
    };
    let storage = StorageService::new(config.storage_base_path.clone());

    let mut uploaded_files = Vec::new();
    let mut errors = Vec::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = match field.file_name() {
            Some(n) => n.to_string(),
            None => {
                errors.push("Missing file name".to_string());
                continue;
            }
        };

        let data = match field.bytes().await {
            Ok(d) => d,
            Err(e) => {
                errors.push(format!("Failed to read {}: {}", name, e));
                continue;
            }
        };

        if data.is_empty() {
            errors.push(format!("{} is empty", name));
            continue;
        }

        let temp_path = format!("/tmp/{}", uuid::Uuid::new_v4());
        if let Err(e) = tokio::fs::write(&temp_path, &data).await {
            errors.push(format!("Failed to write {}: {}", name, e));
            continue;
        }

        let file = match tokio::fs::File::open(&temp_path).await {
            Ok(f) => f,
            Err(e) => {
                errors.push(format!("Failed to open {}: {}", name, e));
                let _ = tokio::fs::remove_file(&temp_path).await;
                continue;
            }
        };

        let (storage_path, checksum, size_bytes) =
            match storage.upload_file(&claims.sub, &name, file).await {
                Ok(res) => res,
                Err(e) => {
                    errors.push(format!("Failed to store {}: {}", name, e));
                    let _ = tokio::fs::remove_file(&temp_path).await;
                    continue;
                }
            };

        let _ = tokio::fs::remove_file(&temp_path).await;

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
            errors.push(format!("Database error for {}: {}", name, e));
            continue;
        }

        println!(
            "Uploaded: {} ({} bytes) by {}",
            name, size_bytes, claims.sub
        );
        uploaded_files.push(name);
    }

    if !uploaded_files.is_empty() {
        let res = json!({
            "status": "success",
            "uploaded": uploaded_files,
            "errors": errors,
            "message": format!("Uploaded {} files", uploaded_files.len())
        });
        (StatusCode::OK, Json(res)).into_response()
    } else {
        let res = json!({
            "status": "error",
            "errors": errors,
            "message": "No files were uploaded successfully"
        });
        (StatusCode::BAD_REQUEST, Json(res)).into_response()
    }
}

pub async fn list_files_handler(
    State(pool): State<sqlx::SqlitePool>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let files = match sqlx::query_as::<_, FileMetadata>(
        "SELECT id, filename, storage_path, owner_username, size_bytes, checksum, uploaded_at FROM files WHERE owner_username = ? ORDER BY uploaded_at DESC"
    )
    .bind(&claims.sub)
    .fetch_all(&pool)
    .await {
        Ok(f) => f,
        Err(e) => {
            eprintln!("List files error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"}))
            ).into_response();
        }
    };

    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "files": files,
            "count": files.len()
        })),
    )
        .into_response()
}
