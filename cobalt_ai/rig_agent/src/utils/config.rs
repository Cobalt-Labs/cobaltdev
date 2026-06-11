use anyhow::Result;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct BasicAgentConfig {
    pub system_prompt: String,
    pub default_model: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ToolAgentConfig {
    pub preamble: String,
    pub default_model: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub log_level: String,
    pub environment: String,
    pub timeout_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProviderConfig {
    pub default_model: String,
    pub temperature: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub app: Option<AppConfig>,
    pub basic_agent: Option<BasicAgentConfig>,
    pub tool_agent: Option<ToolAgentConfig>,
    pub openai: Option<ProviderConfig>,
    pub gemini: Option<ProviderConfig>,
    pub cohere: Option<ProviderConfig>,
    pub anthropic: Option<ProviderConfig>,
}

impl Settings {
    pub fn load() -> Result<Self> {
        let run_dir = std::env::current_dir().unwrap_or_default();
        let paths = vec![
            run_dir.join("configs"),
            run_dir.join("rig_agent/configs"),
            Path::new("/Users/ibrahimhaji/code/cobaltdev/cobalt_ai/configs").to_path_buf(),
        ];

        let mut builder = config::Config::builder();
        let mut found = false;

        for path in paths {
            if path.exists() {
                let defaults = path.join("defaults.toml");
                let agents = path.join("agents.toml");
                let providers = path.join("providers.toml");

                if defaults.exists() {
                    builder = builder.add_source(config::File::from(defaults));
                    found = true;
                }
                if agents.exists() {
                    builder = builder.add_source(config::File::from(agents));
                    found = true;
                }
                if providers.exists() {
                    builder = builder.add_source(config::File::from(providers));
                    found = true;
                }
                if found {
                    break;
                }
            }
        }

        // Fallback to empty if none found
        if !found {
            return Ok(Settings {
                app: None,
                basic_agent: None,
                tool_agent: None,
                openai: None,
                gemini: None,
                cohere: None,
                anthropic: None,
            });
        }

        let settings: Settings = builder.build()?.try_deserialize()?;
        Ok(settings)
    }
}
