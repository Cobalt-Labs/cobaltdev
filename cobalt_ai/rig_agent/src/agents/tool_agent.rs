use rig_core::completion::Prompt;
use rig_core::providers::openai;
use anyhow::Result;
use rig::client::CompletionClient;

pub struct ToolAgent {
    client: openai::Client,
    model: String,
}

impl ToolAgent {
    pub fn new(api_key: String) -> Result<Self> {
        let client = openai::Client::new(&api_key)?;
        Ok(Self {
            client,
            model: "gpt-4o-mini".to_string(),
        })
    }

    pub fn with_model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }

    pub async fn run(&self, user_input: &str) -> Result<String> {
        let agent = self.client.agent(&self.model).build();
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