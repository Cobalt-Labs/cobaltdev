pub mod agents;
pub mod prompts;
pub mod providers;
pub mod tools;
pub mod utils;

pub use agents::basic_chat::BasicAgent;
pub use agents::tool_agent::ToolAgent;
pub use agents::rag_agent::RAGAgent;
pub use tools::{Calculator, FileReader, WebSearch};
pub use rig::providers::openai;
pub use rig::completion::Prompt;