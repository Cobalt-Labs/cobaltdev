use anyhow::Result;
use dotenv::dotenv;
use rig_agent::agents::basic_chat::BasicAgent;
use rig_agent::agents::tool_agent::ToolAgent;
use rig_agent::providers::ProviderClient;
use std::io::{self, Write};
use std::fs::OpenOptions;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt().init();

    let mut provider_client = select_and_initialize_provider()?;

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        println!("\n Rig Agent Lab - Multiple AI Agents Demo\n");
        println!("Active Provider: {}", match &provider_client {
            ProviderClient::OpenAI(_) => "OpenAI",
            ProviderClient::Gemini(_) => "Gemini",
            ProviderClient::Cohere(_) => "Cohere",
            ProviderClient::Anthropic(_) => "Anthropic",
        });
        println!("Select an agent to chat with:");
        println!("1. Basic Chat Agent (Simple conversation)");
        println!("2. Tool-Using Agent (With calculator tool)");
        println!("3. Switch LLM Provider");
        println!("4. Exit\n");

        print!("Choice (1-4): ");
        stdout.flush()?;

        let mut choice = String::new();
        stdin.read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                if run_basic_agent(provider_client.clone()).await? {
                    if let Ok(new_client) = select_and_initialize_provider() {
                        provider_client = new_client;
                    }
                }
            }
            "2" => {
                if run_tool_agent(provider_client.clone()).await? {
                    if let Ok(new_client) = select_and_initialize_provider() {
                        provider_client = new_client;
                    }
                }
            }
            "3" => {
                if let Ok(new_client) = select_and_initialize_provider() {
                    provider_client = new_client;
                }
            }
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please try again"),
        }
    }

    Ok(())
}

fn select_and_initialize_provider() -> Result<ProviderClient> {
    dotenv().ok(); // Reload env to pick up any newly saved keys

    let mut available = Vec::new();
    let openai_key = std::env::var("OPENAI_API_KEY").ok();
    let gemini_key = std::env::var("GEMINI_API_KEY").ok();
    let anthropic_key = std::env::var("ANTHROPIC_API_KEY").ok();
    let cohere_key = std::env::var("COHERE_API_KEY").ok();

    if openai_key.is_some() { available.push(("OpenAI", "OPENAI_API_KEY")); }
    if gemini_key.is_some() { available.push(("Gemini", "GEMINI_API_KEY")); }
    if anthropic_key.is_some() { available.push(("Anthropic", "ANTHROPIC_API_KEY")); }
    if cohere_key.is_some() { available.push(("Cohere", "COHERE_API_KEY")); }

    let choice_configure_new = if available.is_empty() {
        true
    } else {
        println!("\nDetected API keys in environment/`.env`:");
        for (i, (name, var)) in available.iter().enumerate() {
            println!("{}. Use {} ({})", i + 1, name, var);
        }
        println!("{}. Configure/Add a new provider key", available.len() + 1);

        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut choice = String::new();
        let idx = loop {
            print!("\nChoice (1-{}): ", available.len() + 1);
            stdout.flush()?;
            choice.clear();
            stdin.read_line(&mut choice)?;
            if let Ok(num) = choice.trim().parse::<usize>() {
                if num > 0 && num <= available.len() + 1 {
                    break num;
                }
            }
            println!("Invalid choice, please select a valid number.");
        };

        if idx == available.len() + 1 {
            true
        } else {
            let (name, var) = available[idx - 1];
            println!("Using: {}", name);
            let key = std::env::var(var)?;
            return match name {
                "OpenAI" => ProviderClient::new_openai(&key),
                "Gemini" => ProviderClient::new_gemini(&key),
                "Anthropic" => ProviderClient::new_anthropic(&key),
                "Cohere" => ProviderClient::new_cohere(&key),
                _ => unreachable!(),
            };
        }
    };

    if choice_configure_new {
        println!("\nPlease select a provider to configure:");
        println!("1. Gemini (Recommended - free tier at Google AI Studio)");
        println!("2. OpenAI");
        println!("3. Anthropic");
        println!("4. Cohere");
        
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut choice = String::new();
        loop {
            print!("\nChoice (1-4): ");
            stdout.flush()?;
            choice.clear();
            stdin.read_line(&mut choice)?;
            match choice.trim() {
                "1" | "2" | "3" | "4" => break,
                _ => println!("Invalid choice, please select 1-4"),
            }
        }

        let (provider_name, env_var) = match choice.trim() {
            "1" => ("Gemini", "GEMINI_API_KEY"),
            "2" => ("OpenAI", "OPENAI_API_KEY"),
            "3" => ("Anthropic", "ANTHROPIC_API_KEY"),
            "4" => ("Cohere", "COHERE_API_KEY"),
            _ => unreachable!(),
        };

        println!("\nEnter your {} API key: ", provider_name);
        stdout.flush()?;
        let mut api_key = String::new();
        stdin.read_line(&mut api_key)?;
        let api_key = api_key.trim();

        // Save key to .env file
        let mut env_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(".env")?;
        
        writeln!(env_file, "\n{}={}", env_var, api_key)?;
        std::env::set_var(env_var, api_key);
        println!("Saved key to .env file!");

        return match choice.trim() {
            "1" => ProviderClient::new_gemini(api_key),
            "2" => ProviderClient::new_openai(api_key),
            "3" => ProviderClient::new_anthropic(api_key),
            "4" => ProviderClient::new_cohere(api_key),
            _ => unreachable!(),
        };
    }

    unreachable!()
}

fn is_quota_error(err: &anyhow::Error) -> bool {
    let err_str = err.to_string().to_lowercase();
    err_str.contains("insufficient_quota")
        || err_str.contains("quota exceeded")
        || err_str.contains("429")
        || err_str.contains("too many requests")
}

async fn run_basic_agent(provider_client: ProviderClient) -> Result<bool> {
    let agent = BasicAgent::new(provider_client).with_system_prompt(
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
            Err(e) => {
                eprintln!("Error: {}", e);
                if is_quota_error(&e) {
                    println!("\n⚠️  Detected Quota/Rate Limit error from the current provider.");
                    println!("Would you like to switch to or configure another provider now? (y/n)");
                    stdout.flush()?;
                    let mut ans = String::new();
                    stdin.read_line(&mut ans)?;
                    if ans.trim().to_lowercase().starts_with('y') {
                        return Ok(true);
                    }
                }
            }
        }
    }

    Ok(false)
}

async fn run_tool_agent(provider_client: ProviderClient) -> Result<bool> {
    let agent = ToolAgent::new(provider_client);

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
            Err(e) => {
                eprintln!("Error: {}", e);
                if is_quota_error(&e) {
                    println!("\n⚠️  Detected Quota/Rate Limit error from the current provider.");
                    println!("Would you like to switch to or configure another provider now? (y/n)");
                    stdout.flush()?;
                    let mut ans = String::new();
                    stdin.read_line(&mut ans)?;
                    if ans.trim().to_lowercase().starts_with('y') {
                        return Ok(true);
                    }
                }
            }
        }
    }

    Ok(false)
}
