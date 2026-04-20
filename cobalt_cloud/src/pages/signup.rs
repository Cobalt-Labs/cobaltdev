use dioxus::prelude::*;
use crate::services::api;

#[component]
pub fn SignupPage() -> Element {
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm_password = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);
    let mut success = use_signal(|| false);

    let onsubmit = move |_| {
        spawn(async move {
            if username().is_empty() || password().is_empty() {
                error.set(Some("Username and password are required".to_string()));
                return;
            }
            if password() != confirm_password() {
                error.set(Some("Passwords do not match".to_string()));
                return;
            }

            match api::signup(username(), password()).await {
                Ok(_) => {
                    success.set(true);
                    error.set(None);
                }
                Err(e) => error.set(Some(e.to_string())),
            }
        });
    };

    rsx! {
        div { class: "min-h-screen bg-zinc-950 text-white font-sans flex items-center justify-center p-6",
            // Background Decorative Elements
            div { class: "fixed inset-0 overflow-hidden pointer-events-none",
                div { class: "absolute -top-24 -left-24 w-96 h-96 bg-emerald-500/10 rounded-full blur-[120px]" }
                div { class: "absolute top-1/2 -right-24 w-80 h-80 bg-emerald-600/5 rounded-full blur-[100px]" }
            }

            div { class: "w-full max-w-md relative z-10",
                div { class: "bg-zinc-900/40 backdrop-blur-2xl border border-white/5 p-10 rounded-[2.5rem] shadow-2xl",
                    div { class: "text-center mb-10",
                        h1 { class: "text-4xl font-bold tracking-tight mb-3 text-transparent bg-clip-text bg-gradient-to-br from-white to-zinc-500", 
                            "Create Account" 
                        }
                        p { class: "text-zinc-400 font-medium", "Join the secure cloud ecosystem" }
                    }

                    if let Some(msg) = error() {
                        div { class: "mb-6 p-4 bg-red-500/10 border border-red-500/20 rounded-2xl text-red-400 text-sm font-medium animate-in fade-in slide-in-from-top-2",
                            "⚠️ {msg}"
                        }
                    }

                    if success() {
                        div { class: "text-center py-4",
                            div { class: "w-16 h-16 bg-emerald-500/20 rounded-full flex items-center justify-center mx-auto mb-6 border border-emerald-500/30",
                                span { class: "text-2xl", "✅" }
                            }
                            h2 { class: "text-2xl font-bold mb-3", "Success!" }
                            p { class: "text-zinc-400 mb-8", "Your account has been created successfully." }
                            a { 
                                href: "/login",
                                class: "flex items-center justify-center w-full py-4 bg-emerald-500 text-black font-bold rounded-2xl hover:bg-emerald-400 transition-all shadow-[0_10px_40px_-10px_rgba(16,185,129,0.5)]",
                                "Go to Login" 
                            }
                        }
                    } else {
                        form { 
                            class: "space-y-6",
                            onsubmit: move |evt: FormEvent| {
                                evt.prevent_default();
                                onsubmit(());
                            },

                            div {
                                label { class: "block text-xs font-bold text-zinc-500 uppercase tracking-widest mb-2 ml-1", "Username" }
                                input {
                                    class: "w-full bg-zinc-950/50 border border-zinc-800 rounded-2xl px-5 py-4 text-white focus:outline-none focus:border-emerald-500/50 focus:ring-4 focus:ring-emerald-500/10 transition-all font-medium",
                                    placeholder: "Choose a username",
                                    value: "{username}",
                                    oninput: move |e| username.set(e.value()),
                                }
                            }

                            div {
                                label { class: "block text-xs font-bold text-zinc-500 uppercase tracking-widest mb-2 ml-1", "Password" }
                                input {
                                    r#type: "password",
                                    class: "w-full bg-zinc-950/50 border border-zinc-800 rounded-2xl px-5 py-4 text-white focus:outline-none focus:border-emerald-500/50 focus:ring-4 focus:ring-emerald-500/10 transition-all font-medium",
                                    placeholder: "••••••••",
                                    value: "{password}",
                                    oninput: move |e| password.set(e.value()),
                                }
                            }

                            div {
                                label { class: "block text-xs font-bold text-zinc-500 uppercase tracking-widest mb-2 ml-1", "Confirm Password" }
                                input {
                                    r#type: "password",
                                    class: "w-full bg-zinc-950/50 border border-zinc-800 rounded-2xl px-5 py-4 text-white focus:outline-none focus:border-emerald-500/50 focus:ring-4 focus:ring-emerald-500/10 transition-all font-medium",
                                    placeholder: "••••••••",
                                    value: "{confirm_password}",
                                    oninput: move |e| confirm_password.set(e.value()),
                                }
                            }

                            button {
                                r#type: "submit",
                                class: "w-full py-5 bg-gradient-to-r from-emerald-600 to-emerald-500 text-black font-bold rounded-2xl hover:scale-[1.02] active:scale-[0.98] transition-all shadow-[0_20px_50px_-20px_rgba(16,185,129,0.4)]",
                                "Create Account"
                            }
                        }

                        div { class: "mt-10 text-center",
                            p { class: "text-zinc-500 font-medium",
                                "Already have an account? "
                                a { href: "/login", class: "text-emerald-400 hover:text-emerald-300 transition-colors ml-1 font-bold", "Log in" }
                            }
                        }
                    }
                }
            }
        }
    }
}
