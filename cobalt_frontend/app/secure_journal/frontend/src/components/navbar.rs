use dioxus::prelude::*;
use crate::{Route, state::AppState};

#[component]
pub fn Navbar(show_back: bool, show_logout: bool) -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    let nav = navigator();

    let handle_logout = move |_| {
        let mut state = app_state.write();
        state.logged_in = false;
        state.username = String::new();
        nav.push(Route::Login {});
    };

    rsx! {
        nav {
            class: "bg-white shadow-lg",
            div {
                class: "max-w-7xl mx-auto px-4 py-4 flex justify-between items-center",
                h1 { 
                    class: "text-2xl font-bold text-gray-800", 
                    "📔 Secure Journal" 
                }
                div {
                    class: "flex items-center gap-4",
                    if show_back {
                        Link {
                            to: Route::Home {},
                            class: "bg-indigo-500 hover:bg-indigo-600 text-white px-4 py-2 rounded-lg transition",
                            "← Back to Home"
                        }
                    }
                    if show_logout && app_state().logged_in {
                        span { 
                            class: "text-gray-600", 
                            "Welcome, {app_state().username}!" 
                        }
                        button {
                            class: "bg-red-500 hover:bg-red-600 text-white px-4 py-2 rounded-lg transition",
                            onclick: handle_logout,
                            "Logout"
                        }
                    }
                }
            }
        }
    }
}