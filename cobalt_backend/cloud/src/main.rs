use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing_subscriber;

mod routes;
mod cli;
mod config;
mod email;
mod models;
mod services;
mod handlers;
mod middleware;

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
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let cli = Cli::parse();
    let config = config::config::Config::load()?;

    let db_pool = services::database::init_db(&config.db_path).await?;

    match cli.command {
        Commands::Server => {
            println!("Starting Secure Cobalt Backend on http://0.0.0.0:{}", config.server_port);
        
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
            println!("(DB list feature coming soon)");
        }
    }

    Ok(())
}