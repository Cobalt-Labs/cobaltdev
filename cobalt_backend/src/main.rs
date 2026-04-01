use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use clap::Parser;
use anyhow::Result;
use tracing_subscriber;

mod config;           
mod services;
mod cli;              
mod models;

mod email;

// ─────────────────────────────────────────────────────────────
// Your existing models (kept for contact form)
#[derive(Serialize)]
struct ResponseMsg {
    status: String,
}

#[derive(Deserialize)]
struct Contact {
    name: String,
    email: String,
    message: String,
}

// ─────────────────────────────────────────────────────────────
// CLI Parser (from our fireproof plan)
#[derive(Parser)]
#[command(author, version, about = "Cobalt Backend - Cloud Storage + API")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Run the HTTP server (default Axum + future cloud routes)
    Server,
    /// Upload a file to cloud storage via CLI (test HDD)
    Upload {
        file_path: String,
        #[arg(short, long, default_value = "ibrahim3595")]
        user: String,
    },
    /// List files for a user via CLI
    List {
        #[arg(short, long, default_value = "ibrahim3595")]
        user: String,
    },
}

// ─────────────────────────────────────────────────────────────
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let cli = Cli::parse();
    let config = config::config::Config::load()?;

    // Initialize database (with migrations)
    let db_pool = services::database::init_db(&config.db_path).await?;

    match cli.command {
        Commands::Server => {
            println!("🚀 Starting Cobalt Backend server on http://127.0.0.1:{}", config.server_port);

            let app = Router::new()
                .route("/", get(root))
                .route("/contact", post(create_contact))
                // Future cloud routes will go here:
                // .route("/api/upload", post(upload_file_handler))
                // .route("/api/files", get(list_files_handler))
                .layer(CorsLayer::permissive())
                .with_state(db_pool);

            let addr = SocketAddr::from(([127, 0, 0, 1], config.server_port));
            println!("Server running on {}", addr);

            axum::serve(tokio::net::TcpListener::bind(addr).await?, app)
                .await?;
        }

        Commands::Upload { file_path, user } => {
            println!("📤 Uploading {} for user: {}", file_path, user);

            let storage = services::storage::StorageService::new(config.storage_base_path.clone());

            let file = tokio::fs::File::open(&file_path).await?;
            let filename = std::path::Path::new(&file_path)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            let (storage_path, checksum) = storage.upload_file(&user, &filename, file).await?;

            // TODO: Later insert into DB with sqlx
            println!("✅ Upload successful!");
            println!("User: {}", user);
            println!("Storage path: {}", storage_path);
            println!("Checksum (blake3): {}", checksum);
        }

        Commands::List { user } => {
            println!("📋 Listing files for user: {}", user);
            // TODO: Add DB query here later
            println!("(DB list feature coming soon)");
        }
    }

    Ok(())
}

// ─────────────────────────────────────────────────────────────
// Your existing handlers (kept untouched)
async fn root() -> &'static str {
    "Cobalt Backend Running - Cloud Storage Ready"
}

async fn create_contact(
    State(db): State<SqlitePool>,
    Json(payload): Json<Contact>,
) -> Result<Json<ResponseMsg>, (StatusCode, String)> {
    if payload.name.is_empty() || payload.email.is_empty() || payload.message.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "All fields required".into()));
    }

    sqlx::query("INSERT INTO contacts (name, email, message) VALUES (?, ?, ?)")
        .bind(&payload.name)
        .bind(&payload.email)
        .bind(&payload.message)
        .execute(&db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".into()))?;

    // email::send_email(...)  // uncomment when email module is ready

    Ok(Json(ResponseMsg {
        status: "Message saved".into(),
    }))
}