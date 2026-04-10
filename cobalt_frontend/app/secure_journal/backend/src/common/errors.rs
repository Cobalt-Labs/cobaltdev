use std::fmt;
use anyhow::Error as AnyhowError;

#[derive(Debug)]
pub enum _AppError {
    Database(sqlx::Error),

    Password(argon2::password_hash::Error),

    _Input(String),

    _Auth(String),

    Other(String),
}

pub type _Result<T> = std::result::Result<T, _AppError>;

impl fmt::Display for _AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _AppError::Database(e) => write!(f, "Database error: {}", e),
            _AppError::Password(e) => write!(f, "Password hash error: {}", e),
            _AppError::_Input(e) => write!(f, "Input error: {}", e),
            _AppError::_Auth(e) => write!(f, "Authentication error: {}", e),
            _AppError::Other(e) => write!(f, "Unknown error: {}", e),
        }
    }
}

impl std::error::Error for _AppError {}

impl From<sqlx::Error> for _AppError {
    fn from(e: sqlx::Error) -> Self {
        _AppError::Database(e)
    }
}

impl From<argon2::password_hash::Error> for _AppError {
    fn from(e: argon2::password_hash::Error) -> Self {
        _AppError::Password(e)
    }
}

impl From<AnyhowError> for _AppError {
    fn from(e: AnyhowError) -> Self {
        _AppError::Other(e.to_string())
    }
}
