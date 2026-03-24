use axum::{
    Router,
    Json,
    routing::{get, post},
    extract::State,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod email;
mod auth;

#[derive(Deserialize)]
struct AuthPayload {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct ResponseMsg {
    status: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let db = SqlitePool::connect("sqlite:dev.db")
        .await
        .expect("DB connection failed");

    let app = Router::new()
        .route("/", get(root))
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/contact", post(create_contact))
        .layer(CorsLayer::permissive())
        .with_state(db);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Cobalt Backend Running"
}

async fn signup(
    State(db): State<SqlitePool>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<ResponseMsg>, (StatusCode, String)> {

    if payload.email.is_empty() || payload.password.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Missing fields".into()));
    }

    let hashed = auth::hash_password(&payload.password);

    let result = sqlx::query(
        "INSERT INTO users (id, email, password) VALUES (?, ?, ?)"
    )
    .bind(auth::generate_id())
    .bind(&payload.email)
    .bind(&hashed)
    .execute(&db)
    .await;

    match result {
        Ok(_) => Ok(Json(ResponseMsg { status: "User created".into() })),
        Err(_) => Err((StatusCode::BAD_REQUEST, "User already exists".into())),
    }
}

async fn login(
    State(db): State<SqlitePool>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<ResponseMsg>, (StatusCode, String)> {

    let row = sqlx::query("SELECT password FROM users WHERE email = ?")
        .bind(&payload.email)
        .fetch_optional(&db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".into()))?;

    let user = match row {
        Some(user) => user,
        None => return Err((StatusCode::UNAUTHORIZED, "User not found".into())),
    };
    let password: String = user.get("password");

    let valid = auth::verify_password(&password, &payload.password);

    if !valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".into()));
    }

    Ok(Json(ResponseMsg {
        status: "Login successful".into(),
    }))
}

#[derive(Deserialize)]
struct Contact {
    name: String,
    email: String,
    message: String,
}

async fn create_contact(
    State(db): State<SqlitePool>,
    Json(payload): Json<Contact>,
) -> Result<Json<ResponseMsg>, (StatusCode, String)> {

    if payload.name.is_empty() || payload.email.is_empty() || payload.message.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "All fields required".into()));
    }

    sqlx::query(
        "INSERT INTO contacts (name, email, message) VALUES (?, ?, ?)"
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(&payload.message)
    .execute(&db)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".into()))?;

    email::send_email(
        payload.name,
        payload.email,
        payload.message,
    ).await;

    Ok(Json(ResponseMsg {
        status: "Message saved".into(),
    }))
}