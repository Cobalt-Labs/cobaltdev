use anyhow::Result;
use dotenv::dotenv;
use rig_agent::agents::basic_chat::BasicAgent;
use rig_agent::agents::tool_agent::ToolAgent;
use rig_agent::agents::rag_agent::RAGAgent;
use rig_agent::providers::ProviderClient;
use rig_agent::utils::config::Settings;
use rig_agent::utils::logger;
use std::io::{self, Write};
use std::fs::{self, OpenOptions};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // Initialize structured logging
    logger::init_logger();
    logger::log_info("Starting Rig Agent Lab demo application");

    // Load configurations
    let config_settings = Settings::load().unwrap_or_else(|e| {
        eprintln!("Error loading configurations: {}. Using defaults.", e);
        Settings {
            app: None,
            basic_agent: None,
            tool_agent: None,
            openai: None,
            gemini: None,
            cohere: None,
            anthropic: None,
        }
    });

    let mut provider_client = select_and_initialize_provider()?;

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        println!("\n=============================================");
        println!(" Rig Agent Lab - Multiple AI Agents Demo");
        println!("=============================================");
        println!("Active Provider: {}", match &provider_client {
            ProviderClient::OpenAI(_) => "OpenAI",
            ProviderClient::Gemini(_) => "Gemini",
            ProviderClient::Cohere(_) => "Cohere",
            ProviderClient::Anthropic(_) => "Anthropic",
        });
        println!("Select an agent to chat with:");
        println!("1. Basic Chat Agent (Simple conversation)");
        println!("2. Tool-Using Agent (With calculator, file reader, & web search)");
        println!("3. RAG Agent (Retrieval-Augmented Generation)");
        println!("4. Switch LLM Provider");
        println!("5. Exit\n");

        print!("Choice (1-5): ");
        stdout.flush()?;

        let mut choice = String::new();
        stdin.read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                let system_prompt = config_settings.basic_agent.as_ref()
                    .map(|ba| ba.system_prompt.as_str())
                    .unwrap_or("You are a helpful Rust programming assistant. Be concise and practical.");
                if run_basic_agent(provider_client.clone(), system_prompt).await? {
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
                if run_rag_agent(provider_client.clone()).await? {
                    if let Ok(new_client) = select_and_initialize_provider() {
                        provider_client = new_client;
                    }
                }
            }
            "4" => {
                if let Ok(new_client) = select_and_initialize_provider() {
                    provider_client = new_client;
                }
            }
            "5" => {
                logger::log_info("Exiting application");
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

async fn run_basic_agent(provider_client: ProviderClient, system_prompt: &str) -> Result<bool> {
    let agent = BasicAgent::new(provider_client).with_system_prompt(system_prompt);

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

        logger::log_agent_prompt("BasicAgent", input);
        match agent.chat(input).await {
            Ok(response) => {
                logger::log_agent_response("BasicAgent", &response);
                println!("\nAgent > {}\n", response);
            }
            Err(e) => {
                logger::log_error(&format!("Agent error: {}", e));
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
    println!("I have tools! Try: 'What is 15 + 27?', 'Read Cargo.toml', or 'Search for Rig Agent'.");
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

        logger::log_agent_prompt("ToolAgent", input);
        match agent.run(input).await {
            Ok(response) => {
                logger::log_agent_response("ToolAgent", &response);
                println!("\nAgent > {}\n", response);
            }
            Err(e) => {
                logger::log_error(&format!("Agent error: {}", e));
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

async fn run_rag_agent(provider_client: ProviderClient) -> Result<bool> {
    let mut agent = RAGAgent::new(provider_client);

    let doc_dir = "data/documents";
    let _ = fs::create_dir_all(doc_dir);

    let paths = fs::read_dir(doc_dir)?;
    let mut count = 0;
    for _ in paths {
        count += 1;
    }
    if count == 0 {
        let default_doc = format!("{}/about_cobalt.txt", doc_dir);
        let _ = fs::write(&default_doc, "Cobalt AI features state-of-the-art LLM capabilities using the Rig framework. The project allows developerPairings, tool orchestrations, and advanced RAG processing.");
    }

    println!("\nRAG Agent Started!");
    println!("Scanning and loading files from '{}' into context...", doc_dir);
    agent.load_directory_context(doc_dir)?;
    println!("Loaded. Ask me questions about the documents. Type 'back' to return to menu.\n");

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

        logger::log_agent_prompt("RAGAgent", input);
        match agent.ask_with_context(input).await {
            Ok(response) => {
                logger::log_agent_response("RAGAgent", &response);
                println!("\nAgent > {}\n", response);
            }
            Err(e) => {
                logger::log_error(&format!("Agent error: {}", e));
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
