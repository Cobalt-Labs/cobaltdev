pub mod ai;
pub mod filesystem;
pub mod editor;
pub mod project;

// Re-export commonly used types
pub use ai::{LocalLLM, ChatMessage, ChatSession};
pub use filesystem::{FileIndexer, RAGEngine, SearchResult};
pub use editor::{SyntaxHighlighter, CodeEditor};
pub use project::{Workspace, ProjectFile};

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct CobaltStudioConfig {
    pub model_path: String,
    pub max_context_length: usize,
    pub embedding_dimension: usize,
    pub database_path: String,
}

impl Default for CobaltStudioConfig {
    fn default() -> Self {
        Self {
            model_path: "./models/cobalt_model".to_string(),
            max_context_length: 4096,
            embedding_dimension: 768,
            database_path: "./cobalt_studio.db".to_string(),
        }
    }
}

pub fn init(config: CobaltStudioConfig) -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .with_level(true)
        .init();
    
    tracing::info!("Cobalt Studio initialized");
    Ok(())
}