use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

use crate::models::user::User;

pub struct SqliteDatabase {
    pool: SqlitePool,
}

impl SqliteDatabase {
    pub async fn new(url: &str) -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await?;
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            )
            "#,
        )
        .execute(&pool)
        .await?;
        Ok(Self { pool })
    }
    pub async fn create_user(&self, name: &str) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
                INSERT INTO users (name) VALUES (?)
                RETURNING id, name, created_at
            "#,
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
    pub async fn get_user(&self) -> Result<Vec<User>> {
        let users =
            sqlx::query_as::<_, User>("SELECT id, name, created_at FROM users ORDER BY id DESC")
                .fetch_all(&self.pool)
                .await?;

        Ok(users)
    }
}
