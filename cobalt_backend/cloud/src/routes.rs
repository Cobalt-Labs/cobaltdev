use axum::{Router, routing::{get, post}, middleware};
use tower_http::cors::CorsLayer;

use crate::handlers::{auth, files};
use crate::middleware::auth::auth_middleware;

pub fn create_router() -> Router<sqlx::SqlitePool> {
    let protected_routes = Router::new()
        .route("/api/upload", post(files::upload_file_handler))
        .route("/api/files", get(files::list_files_handler))
        .layer(middleware::from_fn(auth_middleware));

    Router::new()
        .route("/", get(|| async { "Cobalt Backend Running - Secure Mode" }))
        .route("/contact", post(auth::create_contact))
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register))
        .nest("/api", protected_routes)
        .layer(CorsLayer::permissive())
}