// src/agents/basic_chat.rs
use anyhow::Result;
use crate::providers::ProviderClient;

#[derive(Clone)]
pub struct BasicAgent {
    client: ProviderClient,
    model: String,
    system_prompt: Option<String>,
}

impl BasicAgent {
    pub fn new(client: ProviderClient) -> Self {
        let model = client.default_model().to_string();
        Self {
            client,
            model,
            system_prompt: None,
        }
    }

    pub fn with_model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }

    pub fn with_system_prompt(mut self, prompt: &str) -> Self {
        self.system_prompt = Some(prompt.to_string());
        self
    }

    pub async fn chat(&self, user_input: &str) -> Result<String> {
        let agent = self.client.build_agent(&self.model, self.system_prompt.as_deref());
        let response = agent.prompt(user_input).await?;
        Ok(response)
    }
}
