use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let mut auth = crate::hooks::use_auth::use_auth_state();
    let nav = use_navigator();
    rsx! {
        nav { class: "sticky top-0 z-50 bg-zinc-900 border-b border-zinc-800 px-8 py-4 flex items-center justify-between",
            // Logo / Wordmark
            div { class: "flex items-center gap-2",
                span { class: "text-white font-semibold text-base tracking-tight", "Cobalt" }
                span { class: "text-zinc-500 text-sm", "Cloud" }
            }

            // Right side
            div { class: "flex items-center gap-4",
                if let Some(user) = auth.read().username.clone() {
                    div { class: "flex items-center gap-4",
                        span { class: "text-sm text-zinc-400", "{user}" }
                        button {
                            class: "text-sm text-zinc-400 hover:text-white transition-colors px-3 py-1.5 rounded-lg hover:bg-zinc-800",
                            onclick: move |_| {
                                auth.set(crate::hooks::use_auth::AuthState::default());
                                nav.push(crate::Route::LoginPage {});
                            },
                            "Sign out"
                        }
                    }
                }
            }
        }
    }
}