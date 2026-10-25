use sqlx::{SqlitePool, migrate::MigrateDatabase};
use anyhow::Result;

pub async fn init_db(db_url: &str) -> Result<SqlitePool> {
    if !sqlx::Sqlite::database_exists(db_url).await? {
        sqlx::Sqlite::create_database(db_url).await?;
        println!("Created new SQLite database");
    }

    let pool = SqlitePool::connect(db_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("Database migrations applied");

    Ok(pool)
}
