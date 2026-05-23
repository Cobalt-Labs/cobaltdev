use axum::{
    extract::DefaultBodyLimit,
    middleware,
    routing::{get, post, options},
    Router,
    response::Response, 
    http::HeaderValue, 
};

use crate::email::send_email_handler;
use crate::handlers::{auth, files};
use crate::middleware::auth::auth_middleware;
use crate::middleware::security::add_security_layers;

async fn options_handler() -> Response {
    let mut response = Response::default();
    response.headers_mut().insert(
        "Access-Control-Allow-Origin",
        HeaderValue::from_static("*"),
    );
    response.headers_mut().insert(
        "Access-Control-Allow-Methods",
        HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"),
    );
    response.headers_mut().insert(
        "Access-Control-Allow-Headers",
        HeaderValue::from_static("Content-Type, Authorization, Accept"),
    );
    response
}

pub fn create_router() -> Router<sqlx::SqlitePool> {
    let protected_routes = Router::new()
        .route("/upload", post(files::upload_file_handler))
        .route("/files", get(files::list_files_handler))
        .layer(middleware::from_fn(auth_middleware))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024));

    let app = Router::new()
        .route("/", get(|| async { "Cobalt Backend Running - Secure Mode" }))
        .route("/api/contact", post(auth::create_contact))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/forgot-password", post(auth::forgot_password))
        .route("/api/send-email", post(send_email_handler))
        .route("/login", post(auth::login))
        .route("/signup", post(auth::register))
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register))
        .route("/api/login", post(auth::login))
        .route("/api/signup", post(auth::register))
        .route("/api/auth/login", options(options_handler))
        .route("/api/auth/register", options(options_handler))
        .route("/login", options(options_handler))
        .route("/signup", options(options_handler))
        .route("/auth/login", options(options_handler))
        .route("/auth/register", options(options_handler))
        .route("/api/upload", options(options_handler))
        .nest("/api", protected_routes);

    add_security_layers(app)
}