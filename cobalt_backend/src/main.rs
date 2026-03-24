use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod email;

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

    sqlx::query("INSERT INTO contacts (name, email, message) VALUES (?, ?, ?)")
        .bind(&payload.name)
        .bind(&payload.email)
        .bind(&payload.message)
        .execute(&db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".into()))?;

    email::send_email(payload.name, payload.email, payload.message).await;

    Ok(Json(ResponseMsg {
        status: "Message saved".into(),
    }))
}
