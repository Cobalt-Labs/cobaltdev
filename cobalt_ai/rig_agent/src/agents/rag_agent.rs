use anyhow::Result;
use rig_core::providers::openai;
use rig_core::completion::Prompt;
use rig::client::CompletionClient;

pub struct RAGAgent {
    client: openai::Client,
    model: String,
    context: Vec<String>,
}

impl RAGAgent {
    pub fn new(api_key: String) -> Result<Self> {
        let client = openai::Client::new(&api_key)?;
        Ok(Self {
            client,
            model: "gpt-4o-mini".to_string(),
            context: Vec::new(),
        })
    }

    pub fn add_context(&mut self, context: &str) {
        self.context.push(context.to_string());
    }

    pub async fn ask_with_context(&self, question: &str) -> Result<String> {
        let context_str = self.context.join("\n\n---\n\n");
        let prompt = format!(
            "Context information:\n{context_str}\n\n\
             Based ONLY on the above context, answer this question: {question}\n\n\
             If the answer cannot be found in the context, say 'I don't have that information.'"
        );
        
        let agent = self.client.agent(&self.model).build();
        let response = agent.prompt(&prompt).await?;
        Ok(response)
    }
}