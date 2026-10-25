use anyhow::{Context, Result};
use dotenv::dotenv;
use rig::providers::openai;
use rig::completion::Prompt;
use rig::client::CompletionClient;
use std::env;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    println!(" Rig Agent - Minimal Barebones Chat");

    let api_key = env::var("OPENAI_API_KEY")
        .context("Missing OPENAI_API_KEY in environment or .env file")?;
    
    let openai_client = openai::Client::new(&api_key)?;
    let agent = openai_client
        .agent("gpt-4o")
        .preamble("You are a helpful and concise AI assistant built with Rust and Rig.")
        .build();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    println!("Agent is ready! Ask me anything (type 'exit' or 'quit' to stop).\n");

    loop {
        print!("You > ");
        stdout.flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            println!("Goodbye!");
            break;
        }

        if input.is_empty() {
            continue;
        }

        match agent.prompt(input).await {
            Ok(response) => {
                println!("Agent > {}\n", response);
            }
            Err(e) => {
                eprintln!("Error calling agent: {}\n", e);
            }
        }
    }

    Ok(())
}
