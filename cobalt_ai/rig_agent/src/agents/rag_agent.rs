// src/agents/rag_agent.rs
use anyhow::Result;
use crate::providers::ProviderClient;
use std::fs;

pub struct RAGAgent {
    client: ProviderClient,
    model: String,
    context: Vec<String>,
}

impl RAGAgent {
    pub fn new(client: ProviderClient) -> Self {
        let model = client.default_model().to_string();
        Self {
            client,
            model,
            context: Vec::new(),
        }
    }

    pub fn with_model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }

    pub fn add_context(&mut self, context: &str) {
        self.context.push(context.to_string());
    }

    pub fn load_directory_context(&mut self, dir_path: &str) -> Result<()> {
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    if let Ok(content) = fs::read_to_string(&path) {
                        let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown");
                        self.add_context(&format!("File: {}\nContents:\n{}", filename, content));
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn ask_with_context(&self, question: &str) -> Result<String> {
        let context_str = self.context.join("\n\n---\n\n");
        let prompt = format!(
            "Context information:\n{context_str}\n\n\
             Based ONLY on the above context, answer this question: {question}\n\n\
             If the answer cannot be found in the context, say 'I don't have that information.'"
        );
        
        let agent = self.client.build_agent(&self.model, None);
        let response = agent.prompt(&prompt).await?;
        Ok(response)
    }
}