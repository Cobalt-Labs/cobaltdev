use dioxus::prelude::*;
use crate::{components::navbar::Navbar, Route, state::AppState};

#[component]
pub fn Home() -> Element {
    let app_state = use_context::<Signal<AppState>>();
    let nav = navigator();

    // Redirect if not logged in
    if !app_state().logged_in {
        nav.push(Route::Login {});
        return rsx! { div {} };
    }

    rsx! {
        div {
            class: "min-h-screen bg-gradient-to-br from-green-50 to-blue-100",
            
            Navbar { show_back: false, show_logout: true }

            // Main content
            div {
                class: "max-w-7xl mx-auto px-4 py-12",
                div {
                    class: "text-center mb-12",
                    h2 { class: "text-5xl font-bold text-gray-800 mb-4", "Your Journal Dashboard" }
                    p { class: "text-xl text-gray-600", "What would you like to do today?" }
                }

                div {
                    class: "grid md:grid-cols-2 gap-8 max-w-4xl mx-auto",
                    
                    // View Entries Card
                    Link {
                        to: Route::Entries {},
                        class: "block",
                        div {
                            class: "bg-white rounded-2xl shadow-xl p-8 hover:shadow-2xl transition transform hover:scale-105 cursor-pointer",
                            div {
                                class: "text-6xl mb-4 text-center",
                                "📖"
                            }
                            h3 { class: "text-2xl font-bold text-gray-800 mb-2 text-center", "View Entries" }
                            p { class: "text-gray-600 text-center", "Browse and read your journal entries" }
                        }
                    }

                    // New Entry Card
                    Link {
                        to: Route::NewEntry {},
                        class: "block",
                        div {
                            class: "bg-white rounded-2xl shadow-xl p-8 hover:shadow-2xl transition transform hover:scale-105 cursor-pointer",
                            div {
                                class: "text-6xl mb-4 text-center",
                                "✍️"
                            }
                            h3 { class: "text-2xl font-bold text-gray-800 mb-2 text-center", "New Entry" }
                            p { class: "text-gray-600 text-center", "Write a new journal entry" }
                        }
                    }
                }
            }
        }
    }
}