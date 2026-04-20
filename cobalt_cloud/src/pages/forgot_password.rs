use dioxus::prelude::*;
use crate::services::api;

#[component]
pub fn ForgotPassPage() -> Element {
    let mut username = use_signal(String::new);
    let mut status_message = use_signal(|| None::<String>);
    let mut is_loading = use_signal(|| false);

    let onsubmit = move |_| {
        spawn(async move {
            if username().is_empty() {
                status_message.set(Some("Please enter your username".to_string()));
                return;
            }

            is_loading.set(true);
            match api::forgot_password(username()).await {
                Ok(msg) => status_message.set(Some(msg)),
                Err(e) => status_message.set(Some(format!("❌ {}", e))),
            }
            is_loading.set(false);
        });
    };

    rsx! {
        div { class: "min-h-screen bg-zinc-950 text-white font-sans flex items-center justify-center p-6",
            // Background Decorative Elements
            div { class: "fixed inset-0 overflow-hidden pointer-events-none",
                div { class: "absolute -bottom-24 -right-24 w-96 h-96 bg-emerald-500/10 rounded-full blur-[120px]" }
            }

            div { class: "w-full max-w-md relative z-10",
                div { class: "bg-zinc-900/40 backdrop-blur-2xl border border-white/5 p-10 rounded-[2.5rem] shadow-2xl",
                    div { class: "text-center mb-10",
                        div { class: "w-16 h-16 bg-zinc-800 rounded-full flex items-center justify-center mx-auto mb-6 border border-white/5",
                            span { class: "text-2xl", "🔑" }
                        }
                        h1 { class: "text-3xl font-bold tracking-tight mb-3", "Reset Password" }
                        p { class: "text-zinc-400 font-medium", "Enter your username to receive instructions" }
                    }

                    if let Some(msg) = status_message() {
                        div { class: "mb-8 p-4 bg-emerald-500/10 border border-emerald-500/20 rounded-2xl text-emerald-400 text-sm font-medium animate-in fade-in zoom-in",
                            "ℹ️ {msg}"
                        }
                    }

                    form { 
                        class: "space-y-6",
                        onsubmit: move |evt: FormEvent| {
                            evt.prevent_default();
                            onsubmit(());
                        },

                        div {
                            label { class: "block text-xs font-bold text-zinc-500 uppercase tracking-widest mb-2 ml-1", "Username" }
                            input {
                                class: "w-full bg-zinc-950/50 border border-zinc-800 rounded-2xl px-5 py-4 text-white focus:outline-none focus:border-emerald-500/50 transition-all font-medium",
                                placeholder: "Enter your username",
                                value: "{username}",
                                oninput: move |e| username.set(e.value()),
                            }
                        }

                        button {
                            r#type: "submit",
                            disabled: is_loading(),
                            class: "w-full py-5 bg-white text-black font-bold rounded-2xl hover:bg-zinc-200 active:scale-[0.98] transition-all flex items-center justify-center gap-2",
                            if is_loading() {
                                div { class: "w-5 h-5 border-2 border-black/20 border-t-black rounded-full animate-spin" }
                            }
                            "Send Reset Instructions"
                        }
                    }

                    div { class: "mt-10 text-center",
                        a { href: "/login", class: "text-zinc-500 hover:text-white transition-colors font-bold flex items-center justify-center gap-2",
                            "Back to Login"
                        }
                    }
                }
            }
        }
    }
}
