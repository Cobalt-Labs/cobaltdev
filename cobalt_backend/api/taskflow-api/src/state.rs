use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

impl AppState {
    pub async fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let db = SqlitePool::connect(&database_url)
            .await
            .expect("Failed to connect to database");

        sqlx::migrate!().run(&db).await.expect("Failed to run migrations");

        Self { db }
    }
}