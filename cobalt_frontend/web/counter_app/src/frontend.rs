use dioxus::prelude::*;

use crate::models::{NameReq, NameResp};

#[component]
pub fn App() -> Element {
    let mut count = use_signal(|| 0);
    let mut step = use_signal(|| 1);
    let mut name = use_signal(|| String::new());
    let stored_name = use_signal(|| String::new());
    let response_message = use_signal(|| String::new());
    let mut is_loading = use_signal(|| false);

    let color = if *count.read() > 10 {
        "#4ecdc4"
    } else if *count.read() < -5 {
        "#ff6b6b"
    } else {
        "white"
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 100vh; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); font-family: system-ui, -apple-system, sans-serif; padding: 20px;",
            
            div {
                style: "background: rgba(255,255,255,0.1); backdrop-filter: blur(10px); border-radius: 20px; padding: 40px 50px; box-shadow: 0 25px 50px rgba(0,0,0,0.3); text-align: center; border: 1px solid rgba(255,255,255,0.2); min-width: 350px; max-width: 500px;",
                
                // Title
                h1 {
                    style: "color: white; font-size: 28px; font-weight: 300; margin: 0 0 5px 0; letter-spacing: 2px; opacity: 0.9;",
                    "🚀 Dioxus + Axum"
                }
                
                p {
                    style: "color: rgba(255,255,255,0.5); font-size: 14px; margin-bottom: 30px;",
                    "Full-stack Rust counter"
                }

                // Counter Section
                div {
                    style: "margin-bottom: 30px; padding-bottom: 30px; border-bottom: 1px solid rgba(255,255,255,0.1);",
                    
                    div {
                        style: "font-size: 80px; font-weight: 700; margin: 10px 0; transition: color 0.3s ease; font-variant-numeric: tabular-nums; text-shadow: 0 2px 10px rgba(0,0,0,0.2); color: {color};",
                        "{count}"
                    }

                    div {
                        style: "display: flex; gap: 15px; justify-content: center; margin-bottom: 15px;",
                        
                        button {
                            style: "width: 60px; height: 60px; border-radius: 50%; border: none; background: #ff6b6b; color: white; font-size: 30px; font-weight: 700; cursor: pointer; transition: all 0.2s; box-shadow: 0 4px 15px rgba(0,0,0,0.2);",
                            onclick: move |_| count -= step(),
                            "-"
                        }
                        
                        button {
                            style: "width: 60px; height: 60px; border-radius: 50%; border: none; background: #4ecdc4; color: white; font-size: 30px; font-weight: 700; cursor: pointer; transition: all 0.2s; box-shadow: 0 4px 15px rgba(0,0,0,0.2);",
                            onclick: move |_| count += step(),
                            "+"
                        }
                    }

                    div {
                        style: "display: flex; gap: 8px; justify-content: center; align-items: center;",
                        
                        span {
                            style: "color: rgba(255,255,255,0.6); font-size: 13px;",
                            "Step:"
                        }
                        
                        button {
                            style: "padding: 4px 12px; border: 1px solid rgba(255,255,255,0.2); background: transparent; color: white; border-radius: 4px; cursor: pointer; font-size: 13px; transition: all 0.2s;",
                            onclick: move |_| step.set(1),
                            "1"
                        }
                        
                        button {
                            style: "padding: 4px 12px; border: 1px solid rgba(255,255,255,0.2); background: transparent; color: white; border-radius: 4px; cursor: pointer; font-size: 13px; transition: all 0.2s;",
                            onclick: move |_| step.set(5),
                            "5"
                        }
                        
                        button {
                            style: "padding: 4px 12px; border: 1px solid rgba(255,255,255,0.2); background: transparent; color: white; border-radius: 4px; cursor: pointer; font-size: 13px; transition: all 0.2s;",
                            onclick: move |_| step.set(10),
                            "10"
                        }
                        
                        button {
                            style: "padding: 4px 12px; border: 1px solid rgba(255,255,255,0.2); background: transparent; color: #ff6b6b; border-radius: 4px; cursor: pointer; font-size: 13px; transition: all 0.2s;",
                            onclick: move |_| count.set(0),
                            "Reset"
                        }
                    }
                }

                // Name Storage Section
                div {
                    style: "margin-top: 10px;",
                    
                    h2 {
                        style: "color: white; font-size: 18px; font-weight: 400; margin: 0 0 15px 0; opacity: 0.8;",
                        "📝 Store Your Name"
                    }

                    div {
                        style: "display: flex; gap: 10px; margin-bottom: 15px;",
                        
                        input {
                            style: "flex: 1; padding: 12px 16px; border: 1px solid rgba(255,255,255,0.2); background: rgba(255,255,255,0.1); color: white; border-radius: 8px; font-size: 16px; outline: none; transition: all 0.2s;",
                            placeholder: "Enter your name...",
                            value: "{name}",
                            oninput: move |e| name.set(e.value()),
                            disabled: is_loading(),
                        }
                        
                        button {
                            style: "padding: 12px 24px; border: none; background: #667eea; color: white; border-radius: 8px; cursor: pointer; font-size: 16px; font-weight: 500; transition: all 0.2s;",
                            onclick: move |_| {
                                let name_to_store = name();
                                if !name_to_store.is_empty() {
                                    is_loading.set(true);
                                    spawn(async move {
                                        store_name(name_to_store).await;
                                    });
                                }
                            },
                            disabled: is_loading() || name().is_empty(),
                            if is_loading() { "⏳" } else { "Store" }
                        }
                    }

                    // Response message
                    if !response_message().is_empty() {
                        div {
                            style: "padding: 12px; background: rgba(78, 205, 196, 0.2); border: 1px solid rgba(78, 205, 196, 0.3); border-radius: 8px; margin-bottom: 10px;",
                            p {
                                style: "color: white; margin: 0; font-size: 14px;",
                                "{response_message}"
                            }
                        }
                    }

                    // Show stored name
                    if !stored_name().is_empty() {
                        div {
                            style: "padding: 10px; background: rgba(255,255,255,0.05); border-radius: 8px;",
                            p {
                                style: "color: rgba(255,255,255,0.6); margin: 0; font-size: 13px;",
                                "Stored: ",
                                span {
                                    style: "color: #4ecdc4; font-weight: 600;",
                                    "{stored_name}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn store_name(name: String) {
    use reqwest::Client;
    
    let client = Client::new();
    let payload = NameReq { name };
    
    // Try using serde_json directly
    let json_payload = serde_json::to_string(&payload).unwrap();
    
    match client
        .post("http://127.0.0.1:3000/api/store-name")
        .header("Content-Type", "application/json")
        .body(json_payload)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                if let Ok(data) = response.json::<NameResp>().await {
                    // Use web-sys for console logging
                    let _ = web_sys::console::log_1(&format!("Response: {}", data.msg).into());
                }
            }
        }
        Err(e) => {
            let _ = web_sys::console::log_1(&format!("Error: {}", e).into());
        }
    }
}