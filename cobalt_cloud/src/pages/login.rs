use dioxus::prelude::*;
use crate::hooks::use_auth;

#[component]
pub fn LoginPage() -> Element {
    let mut auth = use_auth::use_auth();
    let mut username = use_signal(|| "ibrahim3595".to_string());
    let mut password = use_signal(String::new);

    rsx! {
        div { class: "min-h-screen bg-zinc-950 flex items-center justify-center",
            div { class: "bg-zinc-900 p-10 rounded-3xl w-full max-w-md",
                h1 { class: "text-4xl font-bold text-center mb-8", "Welcome Back" }

                input {
                    class: "w-full bg-zinc-800 border border-zinc-700 rounded-2xl px-5 py-4 mb-4 focus:outline-none focus:border-emerald-400",
                    placeholder: "Username",
                    value: "{username}",
                    oninput: move |e| username.set(e.value())
                }

                input {
                    class: "w-full bg-zinc-800 border border-zinc-700 rounded-2xl px-5 py-4 mb-8 focus:outline-none focus:border-emerald-400",
                    r#type: "password",
                    placeholder: "Password",
                    value: "{password}",
                    oninput: move |e| password.set(e.value())
                }

                button {
                    class: "w-full bg-emerald-500 hover:bg-emerald-600 py-4 rounded-2xl font-semibold text-lg transition",
                    onclick: move |_| {
                        auth.set(crate::hooks::use_auth::AuthState {
                            token: Some("fake-jwt-token-for-now".to_string()),
                            username: Some(username.read().clone()),
                        });
                    },
                    "Login to Your Cloud"
                }

                p { class: "text-center text-zinc-500 text-sm mt-6",
                    "Your data stays in your room • Pure Rust"
                }
            }
        }
    }
}