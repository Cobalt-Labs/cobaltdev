use axum::Router;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

mod config;
mod state;
mod routes;
mod handlers;
mod models;
mod services;
mod middleware;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let state = state::AppState::new().await;

    let app = Router::new()
        .nest("/api", routes::create_routes())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "8001".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Test-flow API running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}