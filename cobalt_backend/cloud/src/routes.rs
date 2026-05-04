use axum::{Router, routing::{get, post}, middleware, extract::DefaultBodyLimit};
use tower_http::cors::{CorsLayer, Any};

use crate::handlers::{auth, files};
use crate::middleware::auth::auth_middleware;
use crate::email::send_email_handler;

pub fn create_router() -> Router<sqlx::SqlitePool> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .expose_headers(Any);

    let protected_routes = Router::new()
        .route("/upload", post(files::upload_file_handler))
        .route("/files", get(files::list_files_handler))
        .layer(middleware::from_fn(auth_middleware))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024)); // 1GB limit

    Router::new()
        .route("/", get(|| async { "Cobalt Backend Running - Secure Mode" }))
        .route("/contact", post(auth::create_contact))
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register))
        .route("/auth/forgot-password", post(auth::forgot_password))
        .route("/api/send-email", post(send_email_handler))
        .nest("/api", protected_routes)
        .layer(cors)
}