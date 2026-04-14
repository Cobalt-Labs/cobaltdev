use axum::{Router, routing::{get, post, delete}, middleware};
use crate::handlers;
use crate::middleware::auth::auth_middleware;

pub fn create_routes() -> Router<crate::state::AppState> {
    let protected = Router::new()
        .route("/tasks", post(handlers::task::create_task))
        .route("/tasks", get(handlers::task::get_tasks))
        .route("/tasks/:id", delete(handlers::task::delete_task))
        .layer(middleware::from_fn(auth_middleware));

    Router::new()
        .route("/", get(handlers::health::health))
        .route("/contact", post(handlers::user::register)) // temporary
        .route("/auth/register", post(handlers::user::register))
        .route("/auth/login", post(handlers::user::login))
        .nest("/api", protected)
}