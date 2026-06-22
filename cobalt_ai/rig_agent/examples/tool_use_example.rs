// examples/tool_use_example.rs
use dotenv::dotenv;
use rig_agent::agents::tool_agent::ToolAgent;
use rig_agent::providers::ProviderClient;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let client = ProviderClient::new_openai(&api_key).unwrap();
    let tool_agent = ToolAgent::new(client);

    println!("=== Tool Use Example: Calculator ===");
    let prompt = "Calculate ((15 + 27) * 2) / 4";
    println!("Prompt: {}", prompt);
    match tool_agent.run(prompt).await {
        Ok(res) => println!("Response: {}\n", res),
        Err(e) => eprintln!("Error: {}\n", e),
    }

    println!("=== Tool Use Example: FileReader ===");
    let prompt = "Read the contents of the file Cargo.toml and tell me its package name";
    println!("Prompt: {}", prompt);
    match tool_agent.run(prompt).await {
        Ok(res) => println!("Response: {}\n", res),
        Err(e) => eprintln!("Error: {}\n", e),
    }
}