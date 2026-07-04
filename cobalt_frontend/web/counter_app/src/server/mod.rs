pub mod routes;
pub mod handlers;

use axum::Router;
use tower_http::cors::{Any, CorsLayer};

pub fn create_router() -> Router {
    Router::new()
        .merge(routes::routes())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
}