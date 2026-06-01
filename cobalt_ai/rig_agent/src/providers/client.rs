// src/providers/client.rs
use anyhow::Result;
use rig::providers::{openai, gemini, cohere, anthropic};
use rig::completion::Prompt;
use rig::client::CompletionClient;

#[derive(Clone)]
pub enum ProviderClient {
    OpenAI(openai::Client),
    Gemini(gemini::Client),
    Cohere(cohere::Client),
    Anthropic(anthropic::Client),
}

pub enum BuiltAgent {
    OpenAI(rig::agent::Agent<openai::responses_api::ResponsesCompletionModel>),
    Gemini(rig::agent::Agent<gemini::completion::CompletionModel>),
    Cohere(rig::agent::Agent<cohere::completion::CompletionModel>),
    Anthropic(rig::agent::Agent<anthropic::completion::CompletionModel>),
}

impl BuiltAgent {
    pub async fn prompt(&self, prompt_str: &str) -> Result<String> {
        let response = match self {
            BuiltAgent::OpenAI(agent) => agent.prompt(prompt_str).await?,
            BuiltAgent::Gemini(agent) => agent.prompt(prompt_str).await?,
            BuiltAgent::Cohere(agent) => agent.prompt(prompt_str).await?,
            BuiltAgent::Anthropic(agent) => agent.prompt(prompt_str).await?,
        };
        Ok(response)
    }
}

impl ProviderClient {
    pub fn new_openai(api_key: &str) -> Result<Self> {
        Ok(ProviderClient::OpenAI(openai::Client::new(api_key)?))
    }

    pub fn new_gemini(api_key: &str) -> Result<Self> {
        Ok(ProviderClient::Gemini(gemini::Client::new(api_key)?))
    }

    pub fn new_cohere(api_key: &str) -> Result<Self> {
        Ok(ProviderClient::Cohere(cohere::Client::new(api_key)?))
    }

    pub fn new_anthropic(api_key: &str) -> Result<Self> {
        Ok(ProviderClient::Anthropic(anthropic::Client::new(api_key)?))
    }

    pub fn default_model(&self) -> &'static str {
        match self {
            ProviderClient::OpenAI(_) => "gpt-4o-mini",
            ProviderClient::Gemini(_) => "gemini-1.5-flash",
            ProviderClient::Cohere(_) => "command-r",
            ProviderClient::Anthropic(_) => "claude-3-5-sonnet-20241022",
        }
    }

    pub fn build_agent(&self, model: &str, preamble: Option<&str>) -> BuiltAgent {
        match self {
            ProviderClient::OpenAI(client) => {
                let mut builder = client.agent(model);
                if let Some(p) = preamble {
                    builder = builder.preamble(p);
                }
                BuiltAgent::OpenAI(builder.build())
            }
            ProviderClient::Gemini(client) => {
                let mut builder = client.agent(model);
                if let Some(p) = preamble {
                    builder = builder.preamble(p);
                }
                BuiltAgent::Gemini(builder.build())
            }
            ProviderClient::Cohere(client) => {
                let mut builder = client.agent(model);
                if let Some(p) = preamble {
                    builder = builder.preamble(p);
                }
                BuiltAgent::Cohere(builder.build())
            }
            ProviderClient::Anthropic(client) => {
                let mut builder = client.agent(model).max_tokens(1024);
                if let Some(p) = preamble {
                    builder = builder.preamble(p);
                }
                BuiltAgent::Anthropic(builder.build())
            }
        }
    }
}
