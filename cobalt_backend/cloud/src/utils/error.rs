use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum _AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal error")]
    Internal(#[from] anyhow::Error),

    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for _AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            _AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            _AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            _AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            _AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            _AppError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            ),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

// Convenience alias — your handlers return this
pub type _AppResult<T> = Result<T, _AppError>;