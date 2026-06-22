mod db;
mod handlers;
mod models;
mod routes;

use std::net::SocketAddr;
use tracing_subscriber;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .with_level(true)
        .init();

    dotenvy::dotenv().ok();

    let port = std::env::var("SERVER_PORT")
        .unwrap_or("8081".to_string())
        .parse()
        .unwrap_or(8081);

    println!("🟣 Cobalt Multi-DB Backend");
    println!("📦 Databases: SQLite + MySQL + SurrealDB");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let db_manager = db::DatabaseManager::new().await?;

    let app = routes::create_router().with_state(db_manager);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("🚀 Server running on http://localhost:{}", port);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📝 Endpoints:");
    println!("   POST   /api/users        - Create user in all DBs");
    println!("   GET    /api/users        - Get users from specific DB");
    println!("   GET    /api/users/all    - Get users from all DBs");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}
