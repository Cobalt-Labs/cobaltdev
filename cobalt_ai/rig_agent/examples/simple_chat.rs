// examples/simple_chat.rs
use rig_agent::agents::basic_chat::BasicAgent;
use rig_agent::providers::ProviderClient;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    
    let client = ProviderClient::new_openai(&api_key).unwrap();
    let agent = BasicAgent::new(client);
    
    let response = agent.chat("What's the difference between async and sync code?").await.unwrap();
    println!("{}", response);
}