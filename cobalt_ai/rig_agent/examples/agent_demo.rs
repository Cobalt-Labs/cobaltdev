// examples/agent_demo.rs
use dotenv::dotenv;
use rig_agent::agents::basic_chat::BasicAgent;
use rig_agent::agents::tool_agent::ToolAgent;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = std::env::var("OPENAI_API_KEY").unwrap();
    
    println!("=== Basic Agent Demo ===\n");
    
    // FIXED: Use ? or unwrap() after new()
    let basic_agent = BasicAgent::new(api_key.clone())
        .unwrap()
        .with_system_prompt("You are a concise, helpful assistant.");
    
    match basic_agent.chat("Explain async Rust in one sentence").await {
        Ok(response) => println!("Basic Agent: {}\n", response),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    println!("=== Tool Agent Demo ===\n");
    
    // FIXED: Use run() instead of add_tool/run_with_tools
    let tool_agent = ToolAgent::new(api_key).unwrap();
    
    match tool_agent.run("What is 42 * 3?").await {
        Ok(response) => println!("Tool Agent: {}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}