// examples/rag_example.rs
use dotenv::dotenv;
use rig_agent::agents::rag_agent::RAGAgent;
use rig_agent::providers::ProviderClient;
use std::fs;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let client = ProviderClient::new_openai(&api_key).unwrap();

    let mut rag_agent = RAGAgent::new(client);

    let doc_dir = "data/documents";
    let _ = fs::create_dir_all(doc_dir);
    let doc_path = format!("{}/cobalt_info.txt", doc_dir);
    let mock_content = "Cobalt AI Rig Agent is developed by Cobalt Labs. It features multi-agent orchestration, dynamic configurations, and tool usage.";
    let _ = fs::write(&doc_path, mock_content);

    println!("=== RAG Agent Example ===");
    println!("Loading directory context from '{}'...", doc_dir);
    rag_agent.load_directory_context(doc_dir).unwrap();

    let question = "What is Cobalt AI Rig Agent and who developed it?";
    println!("Question: {}", question);
    match rag_agent.ask_with_context(question).await {
        Ok(res) => println!("Answer: {}\n", res),
        Err(e) => eprintln!("Error: {}\n", e),
    }

    let _ = fs::remove_file(doc_path);
}