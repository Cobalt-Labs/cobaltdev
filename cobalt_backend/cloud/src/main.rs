use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing_subscriber;

// commit msgs for tomorrow
// git add cobalt_backend/cloud/src/config.rs
// git add cobalt_backend/cloud/src/routes.rs
// git add cobalt_backend/cloud/src/handlers/files.rs
// git add cobalt_backend/cloud/.env
// git add cobalt_backend/cloud/.gitignore
// git commit -m "fix: Resolve port conflict and upload API errors
// cd /Users/ibrahimhaji/code/cobaltdev
// git add cobalt_backend/cloud/src/middleware/auth.rs
// git add cobalt_backend/cloud/src/middleware/mod.rs
// git add cobalt_backend/cloud/src/routes.rs


mod cli;
mod config;
mod email;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;

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

#[derive(Parser)]
#[command(author, version, about = "Cobalt Backend - Cloud Storage + API")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    Server,
    Upload {
        file_path: String,
        #[arg(short, long, default_value = "ibrahim3595")]
        user: String,
    },
    List {
        #[arg(short, long, default_value = "ibrahim3595")]
        user: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .with_level(true)
        .init();

    dotenvy::dotenv().ok();

    let cli = Cli::parse();
    let config = config::config::Config::load()?;

    let db_pool = services::database::init_db(&config.db_path).await?;

    match cli.command {
        Commands::Server => {
            println!(
                "Starting Cobalt Backend on http://0.0.0.0:{}",
                config.server_port
            );

            let app = routes::create_router().with_state(db_pool);

            let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
            println!("Server listening on {}", addr);

            axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
        }

        Commands::Upload { file_path, user } => {
            println!("Uploading {} for user: {}", file_path, user);

            let storage = services::storage::StorageService::new(config.storage_base_path.clone());

            let file = tokio::fs::File::open(&file_path).await?;
            let filename = std::path::Path::new(&file_path)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            let (storage_path, checksum, _) = storage.upload_file(&user, &filename, file).await?;

            println!("Upload successful!");
            println!("User: {}", user);
            println!("Storage path: {}", storage_path);
            println!("Checksum: {}", checksum);
        }

        Commands::List { user } => {
            println!("Listing files for user: {}", user);

            let storage = services::storage::StorageService::new(config.storage_base_path.clone());
            let user_dir = storage.get_user_dir(&user);

            if !user_dir.exists() {
                println!("No files found for user: {}", user);
            } else {
                let mut entries = tokio::fs::read_dir(user_dir).await?;
                println!("Files:");
                while let Some(entry) = entries.next_entry().await? {
                    let metadata = entry.metadata().await?;
                    if metadata.is_file() {
                        let file_name = entry.file_name();
                        let size = metadata.len();
                        println!("  - {} ({} bytes)", file_name.to_string_lossy(), size);
                    }
                }
            }
        }
    }

    Ok(())
}
