// src/agents/basic_chat.rs
use anyhow::{Ok, Result};
use rig_core::client::CompletionClient;
use rig_core::completion::Prompt;
use rig_core::providers::openai;

#[derive(Clone)]
pub struct BasicAgent {
    client: openai::Client,
    model: String,
    system_prompt: Option<String>,
}

impl BasicAgent {
    pub fn new(api_key: String) -> Result<Self> {
        let client = openai::Client::new(&api_key)?;
        Ok(Self {
            client,
            model: "gpt-4o-mini".to_string(),
            system_prompt: None,
        })
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
        let agent = if let Some(sys_prompt) = &self.system_prompt {
            self.client.agent(&self.model).preamble(sys_prompt).build()
        } else {
            self.client.agent(&self.model).build()
        };

        let response = agent.prompt(user_input).await?;
        Ok(response)
    }
}
