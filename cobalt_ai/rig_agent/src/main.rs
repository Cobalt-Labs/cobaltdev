use anyhow::Result;
use dotenv::dotenv;
use rig_agent::agents::basic_chat::BasicAgent;
use rig_agent::agents::tool_agent::ToolAgent;
use std::io::{self, Write};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt().init();

    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set in .env file");

    println!(" Rig Agent Lab - Multiple AI Agents Demo\n");
    println!("Select an agent to chat with:");
    println!("1. Basic Chat Agent (Simple conversation)");
    println!("2. Tool-Using Agent (With calculator tool)");
    println!("3. Exit\n");

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("\nChoice (1-3): ");
        stdout.flush()?;

        let mut choice = String::new();
        stdin.read_line(&mut choice)?;

        match choice.trim() {
            "1" => run_basic_agent(api_key.clone()).await?,
            "2" => run_tool_agent(api_key.clone()).await?,
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please try again"),
        }
    }

    Ok(())
}

async fn run_basic_agent(api_key: String) -> Result<()> {
    let agent = BasicAgent::new(api_key)?.with_system_prompt(
        "You are a helpful Rust programming assistant. Be concise and practical.",
    );

    println!("\nBasic Chat Agent Started!");
    println!("Ask me anything about Rust or programming. Type 'back' to return to menu.\n");

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("You > ");
        stdout.flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;

        let input = input.trim();
        if input == "back" {
            break;
        }

        match agent.chat(input).await {
            Ok(response) => println!("\nAgent > {}\n", response),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}

async fn run_tool_agent(api_key: String) -> Result<()> {
    let agent = ToolAgent::new(api_key)?;

    println!("\nTool-Using Agent Started!");
    println!("I have a calculator tool! Try: 'What is 15 + 27?' or 'Calculate 100 / 4'");
    println!("Type 'back' to return to menu.\n");

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("You > ");
        stdout.flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;

        let input = input.trim();
        if input == "back" {
            break;
        }

        match agent.run(input).await {
            Ok(response) => println!("\nAgent > {}\n", response),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}
