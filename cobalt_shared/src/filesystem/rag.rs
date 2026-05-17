use anyhow::Result;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions, Row};
use std::path::Path;
use walkdir::WalkDir;

pub struct RAGEngine {
    pool: SqlitePool,
}

impl RAGEngine {
    pub async fn new(db_path: &str) -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(db_path)
            .await?;
        
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS documents (
                id INTEGER PRIMARY KEY,
                path TEXT NOT NULL,
                content TEXT NOT NULL,
                embedding BLOB NOT NULL
            )"
        ).execute(&pool).await?;
        
        Ok(Self { pool })
    }
    
    pub async fn index_directory(&self, dir_path: &Path) -> Result<usize> {
        let files: Vec<_> = WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
            .collect();
        
        let mut count = 0;
        for entry in files {
            let content = std::fs::read_to_string(entry.path())?;
            let embedding = vec![0.0; 768]; // Placeholder
            
            sqlx::query(
                "INSERT INTO documents (path, content, embedding) VALUES (?, ?, ?)"
            )
            .bind(entry.path().display().to_string())
            .bind(content)
            .bind(embedding)
            .execute(&self.pool)
            .await?;
            
            count += 1;
        }
        
        Ok(count)
    }
    
    pub async fn search(&self, query: &str, top_k: usize) -> Result<Vec<SearchResult>> {
        let results = sqlx::query("SELECT path, content FROM documents WHERE content LIKE ? LIMIT ?")
            .bind(format!("%{}%", query))
            .bind(top_k as i64)
            .fetch_all(&self.pool)
            .await?;
        
        Ok(results.iter().map(|row| {
            SearchResult {
                path: row.get("path"),
                snippet: row.get::<String, _>("content").chars().take(200).collect(),
                relevance: 1.0,
            }
        }).collect())
    }
}

pub struct SearchResult {
    pub path: String,
    pub snippet: String,
    pub relevance: f32,
}