use dioxus::prelude::*;
use std::time::Duration;
use crate::{components::navbar::Navbar, Route, state::AppState};

#[component]
pub fn Entries() -> Element {
    let app_state = use_context::<Signal<AppState>>();
    let nav = navigator();
    let loading = use_signal(|| true);

    // Redirect if not logged in
    if !app_state().logged_in {
        nav.push(Route::Login {});
        return rsx! { div {} };
    }

    // Simulate a loading delay
    use_effect(move || {
        spawn({
            let mut loading = loading.clone();
            async move {
                #[cfg(target_arch = "wasm32")]
                gloo_timers::future::sleep(Duration::from_secs(1)).await;
                #[cfg(not(target_arch = "wasm32"))]
                async_std::task::sleep(Duration::from_secs(1)).await;

                loading.set(false);
            }
        });
    });

    rsx! {
        div {
            class: "min-h-screen bg-gradient-to-br from-green-50 to-blue-100",

            Navbar { show_back: true, show_logout: false }

            div {
                class: "max-w-4xl mx-auto px-4 py-12",
                h2 { class: "text-4xl font-bold text-gray-800 mb-8", "Your Journal Entries" }

                if loading() {
                    div { 
                        class: "text-center py-12 text-gray-600 text-xl",
                        "Loading entries..." 
                    }
                } else if app_state().entries.is_empty() {
                    div {
                        class: "bg-white rounded-2xl shadow-xl p-12 text-center",
                        p { class: "text-gray-600 text-xl mb-4", "No entries yet" }
                        Link {
                            to: Route::NewEntry {},
                            class: "inline-block bg-indigo-600 hover:bg-indigo-700 text-white px-6 py-3 rounded-lg transition",
                            "Write your first entry"
                        }
                    }
                } else {
                    div {
                        class: "space-y-6",
                        for entry in &app_state().entries {
                            div {
                                key: "{entry.id.as_ref().unwrap_or(&String::from(\"unknown\"))}",
                                class: "bg-white rounded-2xl shadow-xl p-6 hover:shadow-2xl transition",
                                h3 { class: "text-2xl font-bold text-gray-800 mb-2", "{entry.title}" }
                                p { class: "text-gray-600 mb-4", "{entry.content}" }
                                div {
                                    class: "flex justify-between items-center",
                                    div {
                                        class: "flex gap-2 flex-wrap",
                                        for tag in &entry.tags {
                                            span {
                                                key: "{tag}",
                                                class: "bg-indigo-100 text-indigo-800 px-3 py-1 rounded-full text-sm",
                                                "{tag}"
                                            }
                                        }
                                    }
                                    span { class: "text-gray-500 text-sm", "{entry.created_at}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
