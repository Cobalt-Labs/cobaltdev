use axum::{Router, routing::{get, delete}};
use sqlx::SqlitePool;
use crate::handlers::*;

pub fn create_routes(pool: SqlitePool) -> Router {
    Router::new()
    .route("/todos", get(get_todos).post(create_todo))
    .route("/todos/{id}", delete(delete_todo))
    .with_state(pool)

}

