use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserReq {
    pub name: String,
}