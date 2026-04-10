use anyhow::Result;
use sqlx::SqlitePool;
// use std::{env, path::PathBuf};

pub type DbPool = SqlitePool;

pub async fn connect() -> Result<DbPool> {
    // let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // path.push("data/journal.db");
    // let url = format!("sqlite: {}", path.display());
    // let pool = SqlitePool::connect(&url).await?;

    let pool = SqlitePool::connect("sqlite:data/journal.db?mode=rwc").await?;
    Ok(pool)
}

