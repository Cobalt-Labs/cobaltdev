use axum::{
    Router,
    http::Method,
    routing::{get, post},
};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers;

pub fn create_router() -> Router<crate::db::DatabaseManager> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any)
        .allow_origin(Any);

    Router::new()
        .route("/", get(|| async { "Cobalt Multi-DB API" }))
        .route("/api/users", post(handlers::users::create_user))
        .route("/api/users", get(handlers::users::get_users))
        .route("/api/users/all", get(handlers::users::get_all_users))
        .layer(cors)
}
