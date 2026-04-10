use sqlx::{SqlitePool};
use std::env;

pub async fn init_db() -> SqlitePool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to DB")
}
