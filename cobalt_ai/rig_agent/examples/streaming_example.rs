// examples/streaming_example.rs
use dotenv::dotenv;
use rig_agent::providers::ProviderClient;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let client = ProviderClient::new_openai(&api_key).unwrap();
    
    println!("=== Streaming Simulation Example ===");
    let agent = client.build_agent("gpt-4o-mini", None);
    let prompt = "Count from 1 to 5, one number per line";
    println!("Prompt: {}\n", prompt);

    match agent.prompt(prompt).await {
        Ok(res) => {
            for word in res.split_whitespace() {
                print!("{} ", word);
                let _ = std::io::Write::flush(&mut std::io::stdout());
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            println!();
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}