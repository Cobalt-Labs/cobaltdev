use dioxus::prelude::*;
use cobalt_shared::ai::OllamaClient;

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    println!("Cobalt Studio starting...");
    
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut chat_messages = use_signal(|| Vec::<(String, String)>::new());
    let mut input_message = use_signal(|| String::new());
    let mut is_loading = use_signal(|| false);
    let model_name = use_signal(|| "deepseek-coder:6.7b".to_string());
    
    let client = use_signal(|| OllamaClient::new(model_name()));
    
    // Send message handler
    let mut send_message = move || {
        let msg = input_message();
        if msg.is_empty() {
            return;
        }
        
        chat_messages.write().push(("user".to_string(), msg.clone()));
        
        input_message.set(String::new());
        is_loading.set(true);
        
        spawn(async move {
            let response = client().generate(&msg).await;
            
            if let Ok(response_text) = response {
                chat_messages.write().push(("assistant".to_string(), response_text));
            } else {
                chat_messages.write().push(("assistant".to_string(), "Error: Could not get response".to_string()));
            }
            
            is_loading.set(false);
        });
    };
    
    rsx! {
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
                    "Model: {model_name()}"
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
                for (role, content) in chat_messages() {
                    if role == "user" {
                        div {
                            style: "
                                display: flex;
                                justify-content: flex-end;
                                margin: 0.5rem 0;
                            ",
                            div {
                                style: "
                                    background: #89b4fa;
                                    padding: 0.5rem 1rem;
                                    border-radius: 12px;
                                    max-width: 70%;
                                    white-space: pre-wrap;
                                    color: #1e1e2e;
                                ",
                                strong { "You: " }
                                span { "{content}" }
                            }
                        }
                    } else {
                        div {
                            style: "
                                display: flex;
                                justify-content: flex-start;
                                margin: 0.5rem 0;
                            ",
                            div {
                                style: "
                                    background: #313244;
                                    padding: 0.5rem 1rem;
                                    border-radius: 12px;
                                    max-width: 70%;
                                    white-space: pre-wrap;
                                ",
                                strong { "🤖 AI: " }
                                span { "{content}" }
                            }
                        }
                    }
                }
            }
            
            // Loading indicator
            if is_loading() {
                div {
                    style: "
                        text-align: center;
                        padding: 0.5rem;
                        color: #89b4fa;
                    ",
                    "Thinking... 🤔"
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
                    oninput: move |e| input_message.set(e.value()),
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
    }
}