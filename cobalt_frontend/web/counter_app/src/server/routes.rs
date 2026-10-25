use super::handlers;

use axum::{routing::get, routing::post, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/api/health", get(handlers::health_check))
        .route("/api/store-name", post(handlers::store_name))
}
