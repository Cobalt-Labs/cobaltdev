use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WebSearch;

#[derive(Deserialize)]
pub struct WebSearchArgs {
    pub query: String,
}

#[derive(Debug, thiserror::Error)]
pub enum WebSearchError {
    #[error("Web search error: {0}")]
    Search(String),
}

impl Tool for WebSearch {
    const NAME: &'static str = "web_search";
    type Error = WebSearchError;
    type Args = WebSearchArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "web_search".to_string(),
            description: "Search the web for information using a query".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The search query (e.g., 'Rust 2024 edition features')"
                    }
                },
                "required": ["query"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let q = args.query.to_lowercase();
        if q.contains("rust") && q.contains("2024") {
            Ok("[Web Search Results]\n1. Rust 2024 Edition introduces changes including RPITIT improvements, and std::gen module.\n2. Enabled via `edition = \"2024\"`.\n3. Guides are at doc.rust-lang.org/edition-guide/rust-2024/index.html.".to_string())
        } else if q.contains("rig") && q.contains("agent") {
            Ok("[Web Search Results]\n1. Rig is a Rust framework for building LLM applications, agents, RAG pipelines, and tool-use.\n2. GitHub: github.com/0xPlaygrounds/rig.\n3. Integrates with OpenAI, Gemini, Cohere, Anthropic.".to_string())
        } else {
            Ok(format!(
                "[Web Search Results for query: \"{}\"]\n1. Found simulated results for: {}\n2. Mock search completed successfully.",
                args.query, args.query
            ))
        }
    }
}
