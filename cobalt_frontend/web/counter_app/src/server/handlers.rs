use axum::{http::StatusCode, response::Json};
use serde_json::json;

use crate::models::{NameReq, NameResp};

pub async fn health_check() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "healthy",
            "message": "Server is running!"
        })),
    )
}

pub async fn store_name(
    Json(payload): Json<NameReq>,
) -> (StatusCode, Json<NameResp>) {
    let response = NameResp {
        msg: format!("Hello, {}! Name stored successfully!", payload.name),
        stored_name: payload.name,
    };
    
    (StatusCode::OK, Json(response))
}