use anyhow::Result;
use dotenvy::dotenv;
use std::env;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Config {
    pub storage_base_path: String,
    pub db_path: String,
    pub server_port: u16,
    pub default_user: String,
    pub jwt_secret: String,
    pub encryption_key: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        dotenv().ok();

        let storage_base_path = env::var("STORAGE_BASE_PATH").unwrap_or_else(|_| {
            let home = env::var("HOME").unwrap_or_else(|_| "/Users/ibrahimhaji".to_string());
            format!("{}/cloud_storage", home)
        });

        println!("Using storage path: {}", storage_base_path);

        if !Path::new(&storage_base_path).exists() {
            std::fs::create_dir_all(&storage_base_path)
                .map_err(|e| anyhow::anyhow!("Failed to create storage dir: {}", e))?;
            println!("Created storage directory: {}", storage_base_path);
        }

        let user_dir = format!("{}/users/ibrahim3595", storage_base_path);
        if !Path::new(&user_dir).exists() {
            std::fs::create_dir_all(&user_dir)
                .map_err(|e| anyhow::anyhow!("Failed to create user dir: {}", e))?;
            println!("Created user directory for ibrahim3595");
        }

        let config = Self {
            storage_base_path,
            db_path: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://cobalt.db?mode=rwc".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or("8080".to_string())
                .parse()
                .map_err(|e| anyhow::anyhow!("Invalid SERVER_PORT: {}", e))?,
            default_user: "ibrahim3595".to_string(),
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| {
                eprintln!(
                    "WARNING: Using default JWT secret. Set JWT_SECRET in .env for production!"
                );
                "super-secret-change-in-production-32-chars-min".to_string()
            }),
            encryption_key: env::var("ENCRYPTION_KEY").unwrap_or_else(|_| {
                eprintln!("WARNING: Using default encryption key. Set ENCRYPTION_KEY in .env!");
                "32-byte-secret-key-for-aes-gcm-123456".to_string()
            }),
        };

        if config.jwt_secret.len() < 32 {
            eprintln!("JWT_SECRET is too short! Use at least 32 characters in production.");
        }
        if config.encryption_key.len() != 32 {
            eprintln!("ENCRYPTION_KEY should be exactly 32 bytes for AES-GCM");
        }

        Ok(config)
    }
}
