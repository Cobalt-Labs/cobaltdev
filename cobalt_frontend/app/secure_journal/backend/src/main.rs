mod auth;
mod common;

mod db;
mod helpers;
mod models;
mod router;

use common::utils::main_menu;
use db::DbPool;
use tokio::net::TcpListener;

use std::sync::Arc;
use axum::serve;

use crate::router::create_router;

pub struct AppState {
    pub db: DbPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("CWD = {:?}", std::env::current_dir()?);
    let db = db::connect().await?;
    let state = Arc::new(AppState { db });

    tokio::spawn(start_server(state.clone()));

    main_menu(&state.db).await;

    Ok(())
    
}

pub async fn start_server(state: Arc<AppState>) {
    let app = create_router(state);

    println!("HTTP API running on http://127.0.0.1:8000");

    let listener = TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("Failed to bind to port 8000");

    serve(listener, app)
        .await
        .expect("Server crashed");
}

