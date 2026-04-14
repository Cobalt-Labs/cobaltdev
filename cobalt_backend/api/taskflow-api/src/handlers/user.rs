use axum::{extract::State, Json, http::StatusCode};
use crate::state::AppState;
use crate::models::user::{User, CreateUser, LoginRequest, LoginResponse};
use crate::services::user_service;
use crate::utils::security;

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Username and password required".into()));
    }

    let hashed = security::hash_password(&payload.password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    user_service::create_user(&state.db, &payload.username, &hashed).await
        .map_err(|e| (StatusCode::CONFLICT, e))?;

    Ok(Json(serde_json::json!({"status": "success", "message": "User registered"})))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let user = user_service::get_user_by_username(&state.db, &payload.username).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let user = match user {
        Some(u) => u,
        None => return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".into())),
    };

    if !security::verify_password(&payload.password, &user.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))? {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".into()));
    }

    let token = user_service::generate_jwt(&user.username)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(LoginResponse { token, username: user.username }))
}