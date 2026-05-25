// examples/simple_chat.rs
use rig_agent::agents::basic_chat::BasicAgent;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = std::env::var("OPENAI_API_KEY").unwrap();
    
    // FIXED: Handle Result with unwrap()
    let agent = BasicAgent::new(api_key).unwrap();
    
    let response = agent.chat("What's the difference between async and sync code?").await.unwrap();
    println!("{}", response);
}