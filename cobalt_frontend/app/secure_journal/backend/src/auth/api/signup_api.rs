use crate::db::DbPool;

use anyhow::{Result, anyhow};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

pub async fn signup_api(pool: &DbPool, username: &str, password: &str) -> Result<()> {
    let salt = SaltString::generate(&mut OsRng);

    let hashed = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow!("argon2 hashing failed: {}", e))?
        .to_string();

    let result = sqlx::query(
        "INSERT INTO users (username, password_hash) 
         VALUES (?, ?)"
    )
    .bind(username)
    .bind(&hashed)
    .execute(pool)
    .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("Failed to create user: {}", e))
    }
}
