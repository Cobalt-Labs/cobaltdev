use crate::common::errors::_Result;
use crate::db::DbPool;
use crate::models::models::User;

use sqlx::Row;
use argon2::{Argon2, PasswordHash, PasswordVerifier};

pub async fn login_api(pool: &DbPool, username: &str, password: &str) -> _Result<Option<User>> {
    let row = sqlx::query(
        "SELECT id, username, password_hash 
         FROM users 
         WHERE username = ?"
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;

    let Some(row) = row else {
        return Ok(None);
    };

    let id: i64 = row.get("id");
    let username: String = row.get("username");
    let hash_str: String = row.get("password_hash");

    let parsed_hash = PasswordHash::new(&hash_str)?;

    let argon2 = Argon2::default();

    let is_ok = argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    if !is_ok {
        return Ok(None);
    }

    Ok(Some(User {
        id,
        username,
        password_hash: hash_str,
    }))
}
