use axum::http::StatusCode;

pub enum AppError {
    Database(String),
    Validation(String),
    Unauthorized,
    Internal,
}

impl From<AppError> for (StatusCode, String) {
    fn from(err: AppError) -> Self {
        match err {
            AppError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".into()),
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".into()),
        }
    }
}