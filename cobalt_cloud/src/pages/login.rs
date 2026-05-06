use dioxus::prelude::*;

#[component]
pub fn LoginPage() -> Element {
    let mut auth = crate::hooks::use_auth::use_auth_state();
    let nav = use_navigator();
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut show_password = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut is_loading = use_signal(|| false);

    // REDIRECTION: If already logged in, go to home
    use_effect(move || {
        if auth.read().token.is_some() {
            nav.push(crate::Route::HomePage {});
        }
    });

    rsx! {
        div { class: "min-h-screen bg-zinc-950 text-white font-sans flex items-center justify-center p-6",
            // Background Decorative Elements
            div { class: "fixed inset-0 overflow-hidden pointer-events-none",
                div { class: "absolute -top-24 -left-24 w-96 h-96 bg-emerald-500/10 rounded-full blur-[120px]" }
                div { class: "absolute bottom-0 right-0 w-80 h-80 bg-emerald-600/5 rounded-full blur-[100px]" }
            }

            div { class: "w-full max-w-md relative z-10",
                div { class: "bg-zinc-900/40 backdrop-blur-2xl border border-white/5 p-12 rounded-[2.5rem] shadow-2xl",
                    h1 { class: "text-4xl font-black text-center mb-2 tracking-tighter text-transparent bg-clip-text bg-gradient-to-br from-white to-zinc-500", "Welcome Back" }
                    p { class: "text-zinc-500 text-center mb-10 font-medium", "Securely access your room cloud" }

                    if let Some(err) = error() {
                        div { class: "mb-6 p-4 bg-red-500/10 border border-red-500/20 rounded-2xl text-red-400 text-sm font-medium animate-in fade-in slide-in-from-top-2",
                            "⚠️ {err}"
                        }
                    }

                    div { class: "space-y-4",
                        div {
                            label { class: "block text-xs font-bold text-zinc-500 uppercase tracking-[0.2em] mb-2 ml-1", "Username" }
                            input {
                                class: "w-full bg-zinc-950/50 border border-white/5 rounded-2xl px-6 py-4 focus:outline-none focus:border-emerald-500/50 focus:ring-4 focus:ring-emerald-500/10 transition-all text-white placeholder-zinc-600 font-medium",
                                placeholder: "Enter username",
                                value: "{username}",
                                oninput: move |e| username.set(e.value())
                            }
                        }

                        div {
                            label { class: "block text-xs font-bold text-zinc-500 uppercase tracking-[0.2em] mb-2 ml-1", "Password" }
                            div { class: "relative",
                                input {
                                    class: "w-full bg-zinc-950/50 border border-white/5 rounded-2xl px-6 py-4 focus:outline-none focus:border-emerald-500/50 focus:ring-4 focus:ring-emerald-500/10 transition-all text-white placeholder-zinc-600 font-medium",
                                    r#type: if show_password() { "text" } else { "password" },
                                    placeholder: "••••••••",
                                    value: "{password}",
                                    oninput: move |e| password.set(e.value())
                                }
                                button {
                                    class: "absolute right-6 top-1/2 -translate-y-1/2 text-zinc-500 hover:text-emerald-400 transition-colors",
                                    onclick: move |_| show_password.set(!show_password()),
                                    if show_password() { "👁️‍🗨️" } else { "👁️" }
                                }
                            }
                        }

                        button {
                            class: "w-full bg-emerald-500 hover:bg-emerald-400 py-5 rounded-2xl font-bold text-lg text-zinc-950 shadow-[0_20px_50px_-15px_rgba(16,185,129,0.4)] hover:shadow-[0_25px_60px_-15px_rgba(16,185,129,0.5)] transition-all transform hover:-translate-y-1 disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none flex items-center justify-center gap-2",
                            disabled: is_loading(),
                            onclick: move |_| {
                                let user = username.read().clone();
                                let pass = password.read().clone();
                                is_loading.set(true);
                                error.set(None);
                                spawn(async move {
                                    match crate::services::api::login(user, pass).await {
                                        Ok(state) => {
                                            auth.set(state);
                                            is_loading.set(false);
                                            nav.push(crate::Route::HomePage {});
                                        }
                                        Err(e) => {
                                            error.set(Some(format!("Login failed: {}", e)));
                                            is_loading.set(false);
                                        }
                                    }
                                });
                            },
                            if is_loading() {
                                span { class: "animate-spin", "🌀" }
                            }
                            "Login to Your Cloud"
                        }
                    }

                    div { class: "mt-10 space-y-4 text-center",
                        p { class: "text-zinc-500 text-sm font-medium",
                            "Don't have an account? "
                            Link { 
                                to: crate::Route::SignupPage {}, 
                                class: "text-emerald-400 hover:text-emerald-300 transition-colors font-bold", 
                                "Sign up" 
                            }
                        }
                        Link { 
                            to: crate::Route::ForgotPassPage {}, 
                            class: "block text-zinc-500 hover:text-zinc-400 text-[10px] font-black uppercase tracking-[0.2em] transition-colors", 
                            "Forgot Password?" 
                        }
                    }

                    p { class: "text-center text-zinc-500/20 text-[9px] mt-12 uppercase tracking-[0.3em] font-black",
                        "End-to-End Secure • Pure Rust"
                    }
                }
            }
        }
    }
}