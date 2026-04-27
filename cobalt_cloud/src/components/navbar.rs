use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let mut auth = crate::hooks::use_auth::use_auth_state();
    let nav = use_navigator();
    rsx! {
        nav { class: "sticky top-0 z-50 bg-zinc-950/40 backdrop-blur-3xl border-b border-white/5 px-10 py-5 flex items-center justify-between",
            div { class: "flex items-center gap-4 group cursor-default",
                div { class: "w-10 h-10 bg-gradient-to-br from-emerald-400 to-emerald-600 rounded-xl flex items-center justify-center shadow-[0_0_20px_-5px_rgba(16,185,129,0.5)] group-hover:rotate-12 transition-transform duration-500",
                    span { class: "text-xl", "☁️" }
                }
                div { class: "flex flex-col -space-y-1",
                    span { class: "text-xl font-black tracking-tighter text-white", "COBALT" }
                    span { class: "text-[10px] text-emerald-500 font-black uppercase tracking-[0.3em]", "Private Cloud" }
                }
            }
            div { class: "flex items-center gap-8",
                div { class: "hidden md:flex items-center gap-3 px-4 py-2 bg-zinc-900/50 rounded-full border border-white/5",
                    div { class: "w-2 h-2 bg-emerald-500 rounded-full animate-pulse shadow-[0_0_10px_rgba(16,185,129,0.8)]" }
                    span { class: "text-xs font-bold text-zinc-400 tracking-wide", "LOCAL NODE ONLINE" }
                }
                
                if let Some(user) = auth.read().username.clone() {
                    div { class: "flex items-center gap-4",
                        span { class: "text-sm font-medium text-zinc-400", "{user}" }
                        button {
                            class: "px-6 py-2.5 bg-white text-zinc-950 rounded-xl font-bold text-sm hover:bg-zinc-200 hover:-translate-y-0.5 active:translate-y-0 transition-all shadow-xl",
                            onclick: move |_| {
                                auth.set(crate::hooks::use_auth::AuthState::default());
                                nav.push(crate::Route::LoginPage {});
                            },
                            "Logout"
                        }
                    }
                }
            }
        }
    }
}