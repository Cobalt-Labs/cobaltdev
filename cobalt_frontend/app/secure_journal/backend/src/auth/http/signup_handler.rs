use std::sync::Arc;
use axum::{extract::State, Json};

use crate::{AppState, auth::api::signup_api};
use super::types::{AuthRequest, AuthResponse};

pub async fn api_signup(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthRequest>,
) -> Json<AuthResponse> {
    match signup_api(&state.db, &payload.username, &payload.password).await {
        Ok(_) => Json(AuthResponse {
            ok: true,
            message: "Signup successful".into(),
        }),

        Err(e) => Json(AuthResponse {
            ok: false,
            message: format!("Signup failed: {}", e),
        }),
    }
}
