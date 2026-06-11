// examples/agent_demo.rs
use dotenv::dotenv;
use rig_agent::agents::basic_chat::BasicAgent;
use rig_agent::agents::tool_agent::ToolAgent;
use rig_agent::providers::ProviderClient;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    
    println!("=== Basic Agent Demo ===\n");
    
    let client = ProviderClient::new_openai(&api_key).unwrap();
    let basic_agent = BasicAgent::new(client.clone())
        .with_system_prompt("You are a concise, helpful assistant.");
    
    match basic_agent.chat("Explain async Rust in one sentence").await {
        Ok(response) => println!("Basic Agent: {}\n", response),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    println!("=== Tool Agent Demo ===\n");
    
    let tool_agent = ToolAgent::new(client);
    
    match tool_agent.run("What is 42 * 3?").await {
        Ok(response) => println!("Tool Agent: {}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}