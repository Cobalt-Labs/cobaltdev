use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FileReader;

#[derive(Deserialize)]
pub struct FileReaderArgs {
    pub path: String,
}

#[derive(Debug, thiserror::Error)]
pub enum FileReaderError {
    #[error("File read error: {0}")]
    Read(String),
}

impl Tool for FileReader {
    const NAME: &'static str = "file_reader";
    type Error = FileReaderError;
    type Args = FileReaderArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "file_reader".to_string(),
            description: "Read contents of a file at the given relative or absolute path".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Path to the file to be read (e.g., configs/agents.toml or data/input.txt)"
                    }
                },
                "required": ["path"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        match fs::read_to_string(&args.path) {
            Ok(content) => Ok(content),
            Err(e) => Err(FileReaderError::Read(format!("Failed to read file {}: {}", args.path, e))),
        }
    }
}
