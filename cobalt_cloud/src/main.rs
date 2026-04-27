use dioxus::prelude::*;
// use dioxus_router::prelude::*;

/// Tasks for upcoming days..
/// enhance drag and drop feature and add upload from Finder for Cloud-GUI-done
/// build, train, analyze the backend for the llm model 
/// create an initial setup for the bootloader and kernel

mod models;
mod services;
mod hooks;
mod components;
mod pages;

use pages::{home::HomePage, login::LoginPage, signup::SignupPage, forgot_password::ForgotPassPage};

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    LoginPage {},
    #[route("/home")]
    HomePage {},
    #[route("/signup")]
    SignupPage {},
    #[route("/forgot-password")]
    ForgotPassPage {},
}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let mut auth_state = crate::hooks::use_auth::use_provide_auth_context();
    crate::hooks::use_files::use_provide_files_context(auth_state);

    // PERSISTENCE: Load from localStorage on startup
    use_effect(move || {
        spawn(async move {
            let res = document::eval(r#"
                let data = localStorage.getItem("cobalt_auth");
                if (data) return data;
                return null;
            "#).await;
            
            if let Ok(data_val) = res {
                if let Some(data_str) = data_val.as_str() {
                    if let Ok(state) = serde_json::from_str::<crate::hooks::use_auth::AuthState>(data_str) {
                        *auth_state.write() = state;
                    }
                }
            }
        });
    });

    // PERSISTENCE: Save to localStorage whenever auth_state changes
    use_effect(move || {
        let auth = auth_state.read().clone();
        spawn(async move {
            if let Ok(json) = serde_json::to_string(&auth) {
                // Escape single quotes for JS compatibility
                let escaped_json = json.replace("'", "\\'");
                let js = format!("localStorage.setItem('cobalt_auth', '{}')", escaped_json);
                let _ = document::eval(&js).await;
            }
        });
    });
    
    rsx! {
        document::Link {
            href: "https://fonts.googleapis.com/css2?family=Outfit:wght@100..900&display=swap",
            rel: "stylesheet"
        }
        document::Stylesheet {
            href: TAILWIND_CSS
        }
        div { class: "font-['Outfit'] select-none",
            Router::<Route> {}
        }
    }
}
