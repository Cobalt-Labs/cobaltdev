use dioxus::prelude::*;
use crate::services::api;

#[component]
pub fn SignupPage() -> Element {
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm_password = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);
    let mut success = use_signal(|| false);
    let mut is_loading = use_signal(|| false);

    let mut onsubmit = move |_| {
        let u = username();
        let p = password();
        let cp = confirm_password();
        is_loading.set(true);
        error.set(None);

        spawn(async move {
            if u.is_empty() || p.is_empty() {
                error.set(Some("Username and password are required".to_string()));
                is_loading.set(false);
                return;
            }
            if p != cp {
                error.set(Some("Passwords do not match".to_string()));
                is_loading.set(false);
                return;
            }

            match api::signup(u, p).await {
                Ok(_) => {
                    success.set(true);
                    error.set(None);
                }
                Err(e) => {
                    error.set(Some(e.to_string()));
                    is_loading.set(false);
                }
            }
        });
    };

    rsx! {
        div { class: "min-h-screen bg-zinc-950 text-white font-sans flex items-center justify-center p-6",
            // Background Decorative Elements
            div { class: "fixed inset-0 overflow-hidden pointer-events-none",
                div { class: "absolute -top-24 -left-24 w-96 h-96 bg-emerald-500/10 rounded-full blur-[120px]" }
                div { class: "absolute bottom-0 right-0 w-80 h-80 bg-blue-600/5 rounded-full blur-[100px]" }
            }

            div { class: "w-full max-w-md relative z-10",
                div { class: "bg-zinc-900/40 backdrop-blur-2xl border border-white/5 p-10 rounded-[2.5rem] shadow-2xl",
                    div { class: "text-center mb-10",
                        h1 { class: "text-4xl font-black tracking-tighter mb-2 text-transparent bg-clip-text bg-gradient-to-br from-white to-zinc-500", 
                            "Create Account" 
                        }
                        p { class: "text-zinc-500 font-medium", "Join the secure cloud ecosystem" }
                    }

                    if let Some(msg) = error() {
                        div { class: "mb-6 p-4 bg-red-500/10 border border-red-500/20 rounded-2xl text-red-400 text-sm font-medium animate-in fade-in slide-in-from-top-2",
                            "⚠️ {msg}"
                        }
                    }

                    if success() {
                        div { class: "text-center py-6 animate-in zoom-in-95 duration-500",
                            div { class: "w-24 h-24 bg-emerald-500/20 rounded-[2rem] flex items-center justify-center mx-auto mb-8 border border-emerald-500/30 shadow-[0_0_50px_-10px_rgba(16,185,129,0.3)]",
                                span { class: "text-4xl", "✅" }
                            }
                            h2 { class: "text-3xl font-black mb-4", "Account Ready" }
                            p { class: "text-zinc-400 mb-10 font-medium", "Your private vault has been provisioned." }
                            Link { 
                                to: crate::Route::LoginPage {},
                                class: "flex items-center justify-center w-full py-5 bg-emerald-500 text-zinc-950 font-bold rounded-2xl hover:bg-emerald-400 transition-all shadow-[0_20px_50px_-10px_rgba(16,185,129,0.5)] transform hover:-translate-y-1",
                                "Back to Login" 
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
                                label { class: "block text-xs font-bold text-zinc-500 uppercase tracking-[0.2em] mb-2 ml-1", "Username" }
                                input {
                                    class: "w-full bg-zinc-950/50 border border-white/5 rounded-2xl px-6 py-4 text-white focus:outline-none focus:border-emerald-500/50 focus:ring-4 focus:ring-emerald-500/10 transition-all font-medium placeholder-zinc-600",
                                    placeholder: "Choose a username",
                                    value: "{username}",
                                    oninput: move |e| username.set(e.value()),
                                }
                            }

                            div {
                                label { class: "block text-xs font-bold text-zinc-500 uppercase tracking-[0.2em] mb-2 ml-1", "Password" }
                                input {
                                    r#type: "password",
                                    class: "w-full bg-zinc-950/50 border border-white/5 rounded-2xl px-6 py-4 text-white focus:outline-none focus:border-emerald-500/50 focus:ring-4 focus:ring-emerald-500/10 transition-all font-medium placeholder-zinc-600",
                                    placeholder: "••••••••",
                                    value: "{password}",
                                    oninput: move |e| password.set(e.value()),
                                }
                            }

                            div {
                                label { class: "block text-xs font-bold text-zinc-500 uppercase tracking-[0.2em] mb-2 ml-1", "Confirm" }
                                input {
                                    r#type: "password",
                                    class: "w-full bg-zinc-950/50 border border-white/5 rounded-2xl px-6 py-4 text-white focus:outline-none focus:border-emerald-500/50 focus:ring-4 focus:ring-emerald-500/10 transition-all font-medium placeholder-zinc-600",
                                    placeholder: "••••••••",
                                    value: "{confirm_password}",
                                    oninput: move |e| confirm_password.set(e.value()),
                                }
                            }

                            button {
                                r#type: "submit",
                                disabled: is_loading(),
                                class: "w-full py-5 bg-gradient-to-br from-emerald-500 to-emerald-600 text-zinc-950 font-bold rounded-2xl hover:shadow-[0_20px_50px_-15px_rgba(16,185,129,0.5)] transition-all transform hover:-translate-y-1 active:scale-[0.98] disabled:opacity-50 flex items-center justify-center gap-2",
                                if is_loading() {
                                    span { class: "animate-spin", "🌀" }
                                }
                                "Register Securely"
                            }
                        }

                        div { class: "mt-10 text-center",
                            p { class: "text-zinc-500 font-medium text-sm",
                                "Already a member? "
                                Link { 
                                    to: crate::Route::LoginPage {}, 
                                    class: "text-emerald-400 hover:text-emerald-300 transition-colors ml-1 font-bold", 
                                    "Log in" 
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
