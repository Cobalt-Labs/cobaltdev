use anyhow::Result;
use dotenvy::dotenv;
use std::env;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Config {
    pub storage_base_path: String,   // e.g. "/mnt/cobalt_storage" or "~/cloud_storage"
    pub db_path: String,             // "sqlite://cobalt.db?mode=rwc"
    pub server_port: u16,
    pub default_user: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        dotenv().ok();

        let storage_base_path = env::var("STORAGE_BASE_PATH")
            .unwrap_or_else(|_| "/mnt/cobalt_storage".to_string());

        // Auto-create base dir (safe for both internal & external after manual mount)
        if !Path::new(&storage_base_path).exists() {
            std::fs::create_dir_all(&storage_base_path)?;
            println!("✅ Created storage directory: {}", storage_base_path);
        }

        let user_dir = format!("{}/users/ibrahim3595", storage_base_path);
        if !Path::new(&user_dir).exists() {
            std::fs::create_dir_all(&user_dir)?;
        }

        Ok(Self {
            storage_base_path,
            db_path: env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://cobalt.db?mode=rwc".to_string()),
            server_port: env::var("SERVER_PORT").unwrap_or("8080".to_string()).parse()?,
            default_user: "ibrahim3595".to_string(),
        })
    }
}