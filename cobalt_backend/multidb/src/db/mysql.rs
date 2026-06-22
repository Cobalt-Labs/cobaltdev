use crate::models::user::User;
use anyhow::Result;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};

pub struct MySqlDatabase {
    pool: MySqlPool,
}

impl MySqlDatabase {
    pub async fn new(url: &str) -> Result<Self> {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await?;
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id INT AUTO_INCREMENT PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&pool)
        .await?;
        Ok(Self { pool })
    }
    pub async fn create_user(&self, name: &str) -> Result<User> {
        let _user = sqlx::query_as::<_, User>(
            r#"
                INSERT INTO users (name) VALUES (?)
            "#,
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await?;

        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, created_at FROM users ORDER BY id DESC LIMIT 1",
        )
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
