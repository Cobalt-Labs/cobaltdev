use axum::{
    Router,
    routing::{get, post},
};
use crate::handlers;

pub fn create_routes() -> Router<crate::state::AppState> {
    Router::new()
        .route("/", get(handlers::health))
        .route("/contact", post(handlers::create_contact))
}