use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { class: "bg-zinc-900 border-b border-zinc-800 px-8 py-4 flex items-center justify-between",
            div { class: "flex items-center gap-3",
                span { class: "text-2xl font-bold text-white", "☁️ Cobalt" }
                span { class: "text-emerald-400 font-mono text-sm", "CLOUD" }
            }
            div { class: "flex items-center gap-6 text-sm",
                span { class: "text-emerald-400", "● Connected to Local HDD" }
                button {
                    class: "px-5 py-2 bg-white text-black rounded-xl font-medium hover:bg-zinc-200 transition",
                    "Logout"
                }
            }
        }
    }
}