use axum::{Json, extract::State, http::StatusCode};
use serde_json::json;
use crate::state::AppState;

pub async fn health() -> &'static str {
    "OK"
}

pub async fn create_contact(
    State(state): State<AppState>,
    Json(payload): Json<crate::models::Contact>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    if payload.name.is_empty() || payload.email.is_empty() || payload.message.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "All fields required".into()));
    }

    sqlx::query("INSERT INTO contacts (name, email, message) VALUES (?, ?, ?)")
        .bind(&payload.name)
        .bind(&payload.email)
        .bind(&payload.message)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)))?;

    // Call your email service
    if let Err(e) = crate::email::send_email(payload.name, payload.email, payload.message).await {
        eprintln!("Email failed: {}", e);
    }

    Ok(Json(json!({"status": "success", "message": "Contact received"})))
}