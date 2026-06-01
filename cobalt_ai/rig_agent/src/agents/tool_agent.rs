// src/agents/tool_agent.rs
use crate::providers::ProviderClient;
use anyhow::Result;

pub struct ToolAgent {
    client: ProviderClient,
    model: String,
}

impl ToolAgent {
    pub fn new(client: ProviderClient) -> Self {
        let model = client.default_model().to_string();
        Self {
            client,
            model,
        }
    }

    pub fn with_model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }

    pub async fn run(&self, user_input: &str) -> Result<String> {
        let agent = self.client.build_agent(&self.model, None);
        let response = agent.prompt(user_input).await?;
        Ok(response)
    }
    
    // Simple calculator function (not as a trait)
    pub fn calculate(&self, operation: &str, a: f64, b: f64) -> Result<String> {
        let result = match operation {
            "add" => a + b,
            "subtract" => a - b,
            "multiply" => a * b,
            "divide" => {
                if b == 0.0 {
                    return Ok("Error: Division by zero".to_string());
                }
                a / b
            },
            _ => return Ok("Invalid operation".to_string()),
        };
        Ok(result.to_string())
    }
}