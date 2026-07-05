use dotenv::dotenv;
use rig::{
    client::{CompletionClient, ProviderClient},
    completion::Prompt,
    providers::deepseek,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let openai_client = deepseek::Client::from_env()?;

    let agent = openai_client
        .agent("deepseek-v3.0-exp")
        .build();

    let response = agent
        .prompt("Explain Rust ownership like I'm 10 years old.")
        .await?;

    println!("{}", response);

    Ok(())
}