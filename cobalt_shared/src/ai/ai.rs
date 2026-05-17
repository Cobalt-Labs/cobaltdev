use anyhow::Result;
use reqwest::Client;
use serde_json::json;

#[derive(Clone)]
pub struct OllamaClient {
    client: Client,
    base_url: String,
    model: String,
}

impl OllamaClient {
    pub fn new(model: String) -> Self {
        Self {
            client: Client::new(),
            base_url: "http://localhost:11434".to_string(),
            model,
        }
    }
    
    pub async fn generate(&self, prompt: &str) -> Result<String> {
        let response = self.client
            .post(format!("{}/api/generate", self.base_url))
            .json(&json!({
                "model": self.model,
                "prompt": prompt,
                "stream": false,
                "options": {
                    "temperature": 0.7,
                    "num_predict": 512
                }
            }))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Ollama error: {}", response.status()));
        }
        
        let result: serde_json::Value = response.json().await?;
        let text = result["response"].as_str().unwrap_or("").to_string();
        
        Ok(text)
    }
    
    pub async fn check_health(&self) -> bool {
        self.client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await
            .is_ok()
    }
}