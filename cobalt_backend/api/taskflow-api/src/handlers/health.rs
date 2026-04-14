use axum::Json;
use serde_json::json;

pub async fn health() -> Json<serde_json::Value> {
    Json(json!({"status": "ok", "message": "TaskFlow API is running"}))
}