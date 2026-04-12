use axum::{extract::State, http::StatusCode, Json};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::rngs::OsRng;
use serde_json::json;
use crate::models::{LoginRequest, LoginResponse, User, Claims};
use crate::{ResponseMsg, Contact};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::Utc;
use sqlx::SqlitePool;

pub async fn login(
    State(pool): State<SqlitePool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Username and password required".into()));
    }

    let user: Option<User> = sqlx::query_as(
        "SELECT id, username, password_hash, created_at FROM users WHERE username = ?"
    )
    .bind(&payload.username)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)))?;

    let user = match user {
        Some(u) => u,
        None => return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".into())),
    };

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Hash error".into()))?;

    if Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash).is_err() {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".into()));
    }

    let claims = Claims::new(user.username.clone());
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(b"super-secret-change-in-production-32-chars-min"),
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(LoginResponse { token, username: user.username }))
}

pub async fn register(
    State(pool): State<SqlitePool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Username and password required".into()));
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .to_string();

    sqlx::query("INSERT INTO users (username, password_hash, created_at) VALUES (?, ?, ?)")
        .bind(&payload.username)
        .bind(&password_hash)
        .bind(Utc::now().to_rfc3339())
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::CONFLICT, format!("User already exists or DB error: {}", e)))?;

    Ok(Json(json!({"status": "User registered successfully"})))
}

pub async fn create_contact(
    State(db): State<SqlitePool>,
    Json(payload): Json<Contact>,
) -> Result<Json<ResponseMsg>, (StatusCode, String)> {
    if payload.name.is_empty() || payload.email.is_empty() || payload.message.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "All fields required".into()));
    }

    sqlx::query("INSERT INTO contacts (name, email, message) VALUES (?, ?, ?)")
        .bind(&payload.name)
        .bind(&payload.email)
        .bind(&payload.message)
        .execute(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)))?;

    if let Err(e) = crate::email::send_email(payload.name, payload.email, payload.message).await {
        eprintln!("Email sending failed: {}", e);
    }

    Ok(Json(ResponseMsg {
        status: "Message received successfully".into(),
    }))
}