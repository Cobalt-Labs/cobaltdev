use dioxus::prelude::*;
use crate::hooks::use_auth;

#[component]
pub fn LoginPage() -> Element {
    let mut auth = use_auth::use_auth();
    let mut username = use_signal(|| "ibrahim3595".to_string());
    let mut password = use_signal(String::new);

    rsx! {
        div { class: "min-h-screen bg-gradient-to-br from-zinc-950 to-zinc-900 flex items-center justify-center text-white font-sans",
            div { class: "bg-zinc-900/60 backdrop-blur-xl border border-white/5 shadow-[0_0_50px_-15px_rgba(0,0,0,0.5)] p-12 rounded-[2rem] w-full max-w-md transform transition-all hover:shadow-[0_0_50px_-15px_rgba(16,185,129,0.1)]",
                h1 { class: "text-4xl font-bold text-center mb-8 tracking-tight", "Welcome Back" }

                input {
                    class: "w-full bg-zinc-950/50 border border-white/5 rounded-2xl px-5 py-4 mb-4 focus:outline-none focus:border-emerald-500/50 focus:ring-1 focus:ring-emerald-500/50 transition-all text-white placeholder-zinc-500",
                    placeholder: "Username",
                    value: "{username}",
                    oninput: move |e| username.set(e.value())
                }

                input {
                    class: "w-full bg-zinc-950/50 border border-white/5 rounded-2xl px-5 py-4 mb-8 focus:outline-none focus:border-emerald-500/50 focus:ring-1 focus:ring-emerald-500/50 transition-all text-white placeholder-zinc-500",
                    r#type: "password",
                    placeholder: "Password",
                    value: "{password}",
                    oninput: move |e| password.set(e.value())
                }

                button {
                    class: "w-full bg-emerald-500 hover:bg-emerald-400 py-4 rounded-2xl font-bold text-lg text-zinc-950 shadow-[0_0_20px_-5px_rgba(16,185,129,0.4)] hover:shadow-[0_0_25px_-5px_rgba(16,185,129,0.6)] transition-all transform hover:-translate-y-0.5",
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