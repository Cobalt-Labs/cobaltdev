use dioxus::prelude::*;
use cobalt_shared::ai::OllamaClient;
use std::sync::Arc;
use tokio::sync::Mutex;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(false)
        .init();
    
    println!("🚀 Cobalt Studio starting...");
    
    // Launch desktop app
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let chat_messages = use_state(cx, || Vec::<(String, String)>::new());
    let input_message = use_state(cx, || String::new());
    let is_loading = use_state(cx, || false);
    let model_name = use_state(cx, || "deepseek-coder:6.7b-instruct-q4_K_M".to_string());
    
    // Create Ollama client
    let client = use_state(cx, || OllamaClient::new(model_name.current().clone()));
    
    // Send message handler
    let send_message = {
        let chat_messages = chat_messages.clone();
        let input_message = input_message.clone();
        let is_loading = is_loading.clone();
        let client = client.clone();
        
        cx.spawn(async move {
            let msg = input_message.current().clone();
            if msg.is_empty() {
                return;
            }
            
            // Add user message
            chat_messages.modify(|msgs| {
                let mut new = msgs.clone();
                new.push(("user".to_string(), msg.clone()));
                new
            });
            
            input_message.set(String::new());
            is_loading.set(true);
            
            // Get AI response
            let response = client.current().generate(&msg).await;
            
            if let Ok(response_text) = response {
                chat_messages.modify(|msgs| {
                    let mut new = msgs.clone();
                    new.push(("assistant".to_string(), response_text));
                    new
                });
            } else {
                chat_messages.modify(|msgs| {
                    let mut new = msgs.clone();
                    new.push(("assistant".to_string(), "Error: Could not get response".to_string()));
                    new
                });
            }
            
            is_loading.set(false);
        });
    };
    
    cx.render(rsx! {
        div {
            style: "
                display: flex;
                flex-direction: column;
                height: 100vh;
                background: #1e1e2e;
                color: #cdd6f4;
                font-family: monospace;
            ",
            
            // Header
            div {
                style: "
                    background: #181825;
                    padding: 1rem;
                    border-bottom: 1px solid #313244;
                ",
                h1 { style: "margin: 0; font-size: 1.5rem;", "🟢 Cobalt Studio" }
                p { style: "margin: 0; font-size: 0.8rem; opacity: 0.8;", 
                    format!("Model: {}", model_name.current())
                }
            }
            
            // Chat area
            div {
                style: "
                    flex: 1;
                    overflow-y: auto;
                    padding: 1rem;
                    display: flex;
                    flex-direction: column;
                    gap: 0.5rem;
                ",
                chat_messages.current().iter().map(|(role, content)| {
                    let is_user = role == "user";
                    let align = if is_user { "flex-end" } else { "flex-start" };
                    let bg = if is_user { "#89b4fa" } else { "#313244" };
                    
                    rsx! {
                        div {
                            style: format!("
                                display: flex;
                                justify-content: {};
                                margin: 0.5rem 0;
                            ", align),
                            div {
                                style: format!("
                                    background: {};
                                    padding: 0.5rem 1rem;
                                    border-radius: 12px;
                                    max-width: 70%;
                                    white-space: pre-wrap;
                                ", bg),
                                if is_user {
                                    rsx! { strong { "You: " } }
                                } else {
                                    rsx! { strong { "🤖 AI: " } }
                                }
                                span { "{content}" }
                            }
                        }
                    }
                })
            }
            
            // Loading indicator
            if *is_loading.current() {
                rsx! {
                    div {
                        style: "
                            text-align: center;
                            padding: 0.5rem;
                            color: #89b4fa;
                        ",
                        "Thinking... 🤔"
                    }
                }
            }
            
            // Input area
            div {
                style: "
                    padding: 1rem;
                    border-top: 1px solid #313244;
                    display: flex;
                    gap: 0.5rem;
                ",
                input {
                    style: "
                        flex: 1;
                        background: #313244;
                        border: none;
                        color: #cdd6f4;
                        padding: 0.75rem;
                        border-radius: 8px;
                        font-family: monospace;
                    ",
                    placeholder: "Ask about code, write Rust functions, or get help...",
                    value: "{input_message}",
                    oninput: move |e| input_message.set(e.value.clone()),
                    onkeypress: move |e| {
                        if e.key() == Key::Enter {
                            send_message();
                        }
                    }
                }
                button {
                    style: "
                        background: #89b4fa;
                        border: none;
                        color: #1e1e2e;
                        padding: 0.75rem 1.5rem;
                        border-radius: 8px;
                        cursor: pointer;
                        font-weight: bold;
                    ",
                    onclick: move |_| send_message(),
                    "Send"
                }
            }
        }
    })
}