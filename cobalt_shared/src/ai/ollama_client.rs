use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone)]
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
                "stream": false
            }))
            .send()
            .await?;
        
        let result: GenerateResponse = response.json().await?;
        Ok(result.response)
    }
    
    pub async fn generate_streaming(
        &self,
        prompt: &str,
        mut callback: impl FnMut(String),
    ) -> Result<()> {
        let response = self.client
            .post(format!("{}/api/generate", self.base_url))
            .json(&json!({
                "model": self.model,
                "prompt": prompt,
                "stream": true
            }))
            .send()
            .await?;
        
        let mut stream = response.bytes_stream();
        use futures_util::StreamExt;
        use std::str;
        
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            if let Ok(text) = str::from_utf8(&chunk) {
                if let Ok(resp) = serde_json::from_str::<GenerateResponse>(text) {
                    callback(resp.response);
                }
            }
        }
        
        Ok(())
    }
    
    pub async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let response = self.client
            .post(format!("{}/api/embeddings", self.base_url))
            .json(&json!({
                "model": self.model,
                "prompt": text
            }))
            .send()
            .await?;
        
        let result: EmbeddingResponse = response.json().await?;
        Ok(result.embedding)
    }
}

#[derive(Debug, Deserialize)]
struct GenerateResponse {
    response: String,
}

#[derive(Debug, Deserialize)]
struct EmbeddingResponse {
    embedding: Vec<f32>,
}