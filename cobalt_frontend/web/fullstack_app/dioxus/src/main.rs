use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[derive(Debug, Clone, PartialEq)]
struct User {
    id: i64,
    name: String,
    created_at: String,
}

fn extract_users(data: serde_json::Value, db_type: &str) -> Vec<User> {
    let mut users = Vec::new();

    if db_type == "all" {
        if let Some(databases) = data.get("databases").and_then(|d| d.as_array()) {
            for db_data in databases {
                if let Some(users_array) = db_data.get("users").and_then(|u| u.as_array()) {
                    for user_json in users_array {
                        if let (Some(id), Some(name), Some(created_at)) = (
                            user_json.get("id").and_then(|v| v.as_i64()),
                            user_json.get("name").and_then(|v| v.as_str()),
                            user_json.get("created_at").and_then(|v| v.as_str()),
                        ) {
                            users.push(User {
                                id,
                                name: name.to_string(),
                                created_at: created_at.to_string(),
                            });
                        }
                    }
                }
            }
        }
    } else if let Some(users_array) = data.get("users").and_then(|u| u.as_array()) {
        for user_json in users_array {
            if let (Some(id), Some(name), Some(created_at)) = (
                user_json.get("id").and_then(|v| v.as_i64()),
                user_json.get("name").and_then(|v| v.as_str()),
                user_json.get("created_at").and_then(|v| v.as_str()),
            ) {
                users.push(User {
                    id,
                    name: name.to_string(),
                    created_at: created_at.to_string(),
                });
            }
        }
    }

    users
}

async fn fetch_users(selected_db: String) -> (Vec<User>, String) {
    let url = if selected_db == "all" {
        "http://localhost:8081/api/users/all".to_string()
    } else {
        format!("http://localhost:8081/api/users?db={}", selected_db)
    };

    match reqwest::Client::new().get(&url).send().await {
        Ok(response) => match response.json::<serde_json::Value>().await {
            Ok(data) => {
                let users = extract_users(data, &selected_db);
                let msg = if users.is_empty() {
                    "❌ No users found".to_string()
                } else {
                    format!("✅ {} user{} loaded", users.len(), if users.len() == 1 { "" } else { "s" })
                };
                (users, msg)
            }
            Err(e) => (Vec::new(), format!("❌ Parse error: {e}")),
        },
        Err(e) => (Vec::new(), format!("❌ Network error: {e}")),
    }
}

#[component]
fn App() -> Element {
    let mut users = use_signal(|| Vec::<User>::new());
    let mut name_input = use_signal(|| String::new());
    let mut status_message = use_signal(|| String::new());
    let mut is_loading = use_signal(|| false);
    let mut selected_db = use_signal(|| "sqlite".to_string());

    // Auto-load users whenever selected_db changes
    let _load_task = use_resource(move || async move {
        is_loading.set(true);
        status_message.set("Loading users...".to_string());
        let (loaded, msg) = fetch_users(selected_db()).await;
        users.set(loaded);
        status_message.set(msg);
        is_loading.set(false);
    });

    // Manual reload helper
    let reload = move || {
        spawn(async move {
            is_loading.set(true);
            status_message.set("Loading users...".to_string());
            let (loaded, msg) = fetch_users(selected_db()).await;
            users.set(loaded);
            status_message.set(msg);
            is_loading.set(false);
        });
    };

    let mut create_user = move || {
        let name = name_input().trim().to_string();
        if name.is_empty() {
            status_message.set("❌ Name cannot be empty".to_string());
            return;
        }
        spawn(async move {
            is_loading.set(true);
            status_message.set("Creating user...".to_string());

            match reqwest::Client::new()
                .post("http://localhost:8081/api/users")
                .header("Content-Type", "application/json")
                .body(format!(r#"{{"name":"{}"}}"#, name))
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        status_message.set(format!("✅ User '{}' created!", name));
                        name_input.set(String::new());
                        // Refresh list for current DB
                        let (loaded, msg) = fetch_users(selected_db()).await;
                        users.set(loaded);
                        status_message.set(msg);
                    } else {
                        status_message.set(format!("❌ Error: {}", response.status()));
                    }
                }
                Err(e) => status_message.set(format!("❌ Network error: {e}")),
            }

            is_loading.set(false);
        });
    };

    // Snapshot reactive values for use in rsx
    let status = status_message();
    let loading = is_loading();
    let current_db = selected_db();
    let user_list = users();

    // Returns pill button style — active = highlighted
    let btn = |db: &str| {
        let active = current_db == db;
        format!(
            "padding: 0.35rem 1rem; border-radius: 20px; font-size: 0.78rem; font-weight: 600; \
             cursor: pointer; transition: all 0.2s ease; border: 1px solid {}; \
             background: {}; color: {};",
            if active { "#f5576c" } else { "rgba(255,255,255,0.1)" },
            if active { "rgba(245,87,108,0.18)" } else { "transparent" },
            if active { "#f5576c" } else { "#8888aa" },
        )
    };

    // Accent colour per DB badge in user rows
    let row_accent = |db: &str| match db {
        "mysql" => "#00758f",
        _ => "#f5576c", // sqlite default
    };

    rsx! {
        div {
            style: "font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; \
                    max-width: 820px; margin: 0 auto; padding: 2rem; \
                    background: #0a0a1a; min-height: 100vh; color: #e0e0e0;",

            // ── Header ───────────────────────────────────────────────
            header {
                style: "text-align: center; padding: 2rem 0; \
                        border-bottom: 1px solid rgba(255,255,255,0.05); margin-bottom: 2rem;",
                h1 {
                    style: "font-size: 2.4rem; font-weight: 700; margin-bottom: 0.4rem; \
                            background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%); \
                            -webkit-background-clip: text; -webkit-text-fill-color: transparent;",
                    "🟣 Cobalt Dioxus"
                }
                p { style: "color: #8888aa; font-size: 0.95rem;", "Pure Rust Full-Stack · SQLite + MySQL" }
                div {
                    style: "display: flex; gap: 0.5rem; justify-content: center; margin-top: 0.8rem;",
                    span {
                        style: "padding: 0.2rem 0.8rem; border-radius: 20px; background: #336791; \
                                color: white; font-size: 0.68rem; font-weight: 700; text-transform: uppercase;",
                        "SQLite"
                    }
                    span {
                        style: "padding: 0.2rem 0.8rem; border-radius: 20px; background: #00758f; \
                                color: white; font-size: 0.68rem; font-weight: 700; text-transform: uppercase;",
                        "MySQL"
                    }
                }
            }

            // ── Add User Card ─────────────────────────────────────────
            div {
                style: "background: rgba(255,255,255,0.03); border-radius: 16px; \
                        padding: 1.6rem; margin-bottom: 1.6rem; \
                        border: 1px solid rgba(255,255,255,0.07);",
                h2 { style: "font-size: 1.1rem; font-weight: 600; margin-bottom: 1rem; color: #ccccff;", "Add User" }
                div {
                    style: "display: flex; gap: 0.8rem;",
                    input {
                        style: "flex: 1; padding: 0.75rem 1rem; border-radius: 10px; \
                                border: 1px solid rgba(255,255,255,0.1); \
                                background: rgba(255,255,255,0.05); \
                                color: #e0e0e0; font-size: 0.95rem; outline: none;",
                        placeholder: "Enter name...",
                        value: "{name_input}",
                        oninput: move |e| name_input.set(e.value()),
                        onkeydown: move |e| {
                            if e.key() == Key::Enter { create_user(); }
                        }
                    }
                    button {
                        style: "padding: 0.75rem 1.8rem; border-radius: 10px; border: none; \
                                font-weight: 600; font-size: 0.95rem; cursor: pointer; \
                                background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%); \
                                color: white; transition: opacity 0.2s;",
                        onclick: move |_| create_user(),
                        disabled: loading,
                        if loading { "⏳ Adding..." } else { "💾 Add User" }
                    }
                }
                if !status.is_empty() {
                    p {
                        style: if status.starts_with("✅") {
                            "margin-top: 0.8rem; padding: 0.5rem 1rem; border-radius: 8px; \
                             font-size: 0.88rem; background: rgba(46,213,115,0.12); \
                             color: #2ed573; border: 1px solid rgba(46,213,115,0.2);"
                        } else {
                            "margin-top: 0.8rem; padding: 0.5rem 1rem; border-radius: 8px; \
                             font-size: 0.88rem; background: rgba(255,71,87,0.12); \
                             color: #ff4757; border: 1px solid rgba(255,71,87,0.2);"
                        },
                        "{status}"
                    }
                }
            }

            // ── Users List Card ───────────────────────────────────────
            div {
                style: "background: rgba(255,255,255,0.03); border-radius: 16px; \
                        padding: 1.6rem; border: 1px solid rgba(255,255,255,0.07);",

                // Header row: title + DB selector pills
                div {
                    style: "display: flex; justify-content: space-between; \
                            align-items: center; margin-bottom: 1.2rem; flex-wrap: wrap; gap: 0.6rem;",
                    h2 { style: "font-size: 1.1rem; font-weight: 600; color: #ccccff; margin: 0;", "👥 Users" }

                    // DB selector pills
                    div {
                        style: "display: flex; gap: 0.4rem; flex-wrap: wrap; align-items: center;",

                        button {
                            id: "btn-sqlite",
                            style: btn("sqlite"),
                            onclick: move |_| { selected_db.set("sqlite".to_string()); reload(); },
                            "🗄 SQLite"
                        }
                        button {
                            id: "btn-mysql",
                            style: btn("mysql"),
                            onclick: move |_| { selected_db.set("mysql".to_string()); reload(); },
                            "🐬 MySQL"
                        }
                        button {
                            id: "btn-all",
                            style: btn("all"),
                            onclick: move |_| { selected_db.set("all".to_string()); reload(); },
                            "📊 All"
                        }
                        // Divider
                        span { style: "width: 1px; height: 1.2rem; background: rgba(255,255,255,0.12); margin: 0 0.2rem;", "" }
                        button {
                            id: "btn-refresh",
                            style: "padding: 0.35rem 0.9rem; border-radius: 20px; font-size: 0.78rem; \
                                    cursor: pointer; border: 1px solid rgba(255,255,255,0.1); \
                                    background: transparent; color: #8888aa; transition: all 0.2s;",
                            onclick: move |_| reload(),
                            "🔄"
                        }
                    }
                }

                // DB label badge
                div {
                    style: "margin-bottom: 0.8rem; display: flex; align-items: center; gap: 0.5rem;",
                    span {
                        style: format!(
                            "padding: 0.15rem 0.7rem; border-radius: 12px; font-size: 0.7rem; \
                             font-weight: 700; text-transform: uppercase; color: white; background: {};",
                            if current_db == "mysql" { "#00758f" }
                            else if current_db == "all" { "#6c4bc4" }
                            else { "#336791" }
                        ),
                        "{current_db}"
                    }
                    span {
                        style: "color: #666688; font-size: 0.78rem;",
                        "· {user_list.len()} user{s}",
                        s: if user_list.len() == 1 { "" } else { "s" }
                    }
                }

                if loading && user_list.is_empty() {
                    div {
                        style: "text-align: center; padding: 2.5rem; color: #666688;",
                        "⏳ Loading..."
                    }
                } else if user_list.is_empty() {
                    div {
                        style: "text-align: center; padding: 2.5rem; color: #666688; font-style: italic;",
                        "📭 No users yet — add one above!"
                    }
                } else {
                    for user in user_list.iter() {
                        div {
                            key: "{user.id}",
                            style: format!(
                                "display: flex; justify-content: space-between; align-items: center; \
                                 padding: 0.65rem 1rem; background: rgba(255,255,255,0.025); \
                                 border-radius: 10px; margin-bottom: 0.45rem; \
                                 border-left: 3px solid {}; transition: background 0.15s;",
                                row_accent(if current_db == "all" { "sqlite" } else { &current_db })
                            ),
                            // Left: avatar + name
                            div {
                                style: "display: flex; align-items: center; gap: 0.6rem;",
                                span {
                                    style: format!(
                                        "width: 2rem; height: 2rem; border-radius: 50%; \
                                         display: flex; align-items: center; justify-content: center; \
                                         font-size: 0.8rem; font-weight: 700; color: white; background: {};",
                                        row_accent(if current_db == "all" { "sqlite" } else { &current_db })
                                    ),
                                    // initials
                                    "{user.name.chars().next().unwrap_or('?').to_uppercase()}"
                                }
                                span { style: "font-weight: 500; color: #d0d0e0;", "{user.name}" }
                            }
                            // Right: meta info
                            div {
                                style: "display: flex; align-items: center; gap: 0.8rem;",
                                span {
                                    style: "font-size: 0.72rem; color: #555577; \
                                            background: rgba(255,255,255,0.04); \
                                            padding: 0.15rem 0.5rem; border-radius: 6px;",
                                    "#{user.id}"
                                }
                                span {
                                    style: "font-size: 0.72rem; color: #555577;",
                                    "{user.created_at}"
                                }
                            }
                        }
                    }
                }
            }

            // ── Footer ────────────────────────────────────────────────
            footer {
                style: "text-align: center; padding-top: 1.5rem; margin-top: 1.5rem; \
                        border-top: 1px solid rgba(255,255,255,0.05); \
                        color: #333355; font-size: 0.78rem;",
                "Built with ❤️ using Rust · Axum · Dioxus · SQLite · MySQL"
            }
        }
    }
}