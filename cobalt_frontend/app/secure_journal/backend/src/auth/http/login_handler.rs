use std::sync::Arc;
use axum::{extract::State, Json};

use crate::{AppState, auth::api::login_api};
use super::types::{AuthRequest, AuthResponse};

pub async fn api_login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthRequest>,
) -> Json<AuthResponse> {
    match login_api(&state.db, &payload.username, &payload.password).await {
        Ok(Some(_user)) => Json(AuthResponse {
            ok: true,
            message: "Login successful".into(),
        }),

        Ok(None) => Json(AuthResponse {
            ok: false,
            message: "Invalid username or password".into(),
        }),

        Err(e) => Json(AuthResponse {
            ok: false,
            message: format!("Server error: {}", e),
        }),
    }
}
