use sqlx::SqlitePool;
use crate::models::user::User;

pub async fn create_user(pool: &SqlitePool, username: &str, password_hash: &str) -> Result<(), String> {
    sqlx::query("INSERT INTO users (username, password_hash, created_at) VALUES (?, ?, ?)")
        .bind(username)
        .bind(password_hash)
        .bind(sqlx::types::chrono::Utc::now())
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to create user: {}", e))?;

    Ok(())
}

pub async fn get_user_by_username(pool: &SqlitePool, username: &str) -> Result<Option<User>, String> {
    let user = sqlx::query_as(
        "SELECT id, username, password_hash, created_at FROM users WHERE username = ?"
    )
    .bind(username)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(user)
}

pub fn generate_jwt(username: &str) -> Result<String, String> {
    let claims = crate::models::user::Claims::new(username.to_string());
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(b"super-secret-change-in-production-32-chars-min"),
    ).map_err(|e| e.to_string())
}