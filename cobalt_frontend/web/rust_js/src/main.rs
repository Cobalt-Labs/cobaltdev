use axum::{
    Json, Router,
    extract::State,
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use std::sync::Arc;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserReq {
    name: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResp<T> {
    pub success: bool,
    pub msg: String,
    pub data: Option<T>,
    pub error: Option<String>,
}

pub struct AppState {
    pub pool: SqlitePool,
}

async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserReq>,
) -> impl IntoResponse {
    if payload.name.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResp::<User> {
                success: false,
                msg: "Name cannot be empty".to_string(),
                data: None,
                error: Some("Validation error".to_string()),
            }),
        );
    }

    match sqlx::query("INSERT INTO users (name) VALUES (?) RETURNING id, name")
        .bind(payload.name.trim())
        .fetch_one(&state.pool)
        .await
    {
        Ok(row) => {
            let user = User {
                id: row.get(0),
                name: row.get(1),
            };

            println!("User created: {} (ID: {})", user.name, user.id);

            (
                StatusCode::CREATED,
                Json(ApiResp {
                    success: true,
                    msg: format!("User '{}' created successfully", user.name),
                    data: Some(user),
                    error: None,
                }),
            )
        }
        Err(e) => {
            eprintln!("❌ Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResp::<User> {
                    success: false,
                    msg: "Failed to create user".to_string(),
                    data: None,
                    error: Some(format!("Database error: {}", e)),
                }),
            )
        }
    }
}

async fn get_users(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match sqlx::query("SELECT id, name FROM users ORDER BY id DESC")
        .fetch_all(&state.pool)
        .await
    {
        Ok(rows) => {
            let users: Vec<User> = rows
                .into_iter()
                .map(|row| User {
                    id: row.get(0),
                    name: row.get(1),
                })
                .collect();

            println!("Fetched {} users", users.len());

            (
                StatusCode::OK,
                Json(ApiResp {
                    success: true,
                    msg: format!("Fetched {} users", users.len()),
                    data: Some(users),
                    error: None,
                }),
            )
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResp::<Vec<User>> {
                    success: false,
                    msg: "Failed to fetch users".to_string(),
                    data: None,
                    error: Some(format!("Database error: {}", e)),
                }),
            )
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .with_level(true)
        .init();

    println!("Rust + Vanilla JS App");

    let pool = SqlitePool::connect("sqlite:./db/rust_js.db?mode=rwc").await?;
    init_db(&pool).await?;

    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/users", post(create_user))
                .route("/users", get(get_users)),
        )
        .fallback_service(ServeDir::new("static"))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                .allow_headers(Any),
        )
        .with_state(Arc::new(AppState { pool }));

    let addr = "0.0.0.0:3000";
    println!("🌐 http://localhost:3000");
    println!("📝 POST /api/users  - Create user");
    println!("📝 GET  /api/users  - Get users");

    axum::serve(
        tokio::net::TcpListener::bind(addr).await?,
        app.into_make_service(),
    )
    .await?;

    Ok(())
}
