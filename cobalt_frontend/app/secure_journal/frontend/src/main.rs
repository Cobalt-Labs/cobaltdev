const _FORCE_TW: &str = "bg-red-500 text-white p-10 flex";

mod api;
mod components;
mod models;
mod pages;
mod state;

use crate::pages::entries::Entries;
use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::new_entry::NewEntry;
use crate::pages::signup::Signup;
use dioxus::prelude::*;
use state::AppState;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Login {},
    #[route("/signup")]
    Signup {},
    #[route("/home")]
    Home {},
    #[route("/entries")]
    Entries {},
    #[route("/new-entry")]
    NewEntry {},
}

// const MAIN_CSS: &str = "/assets/main.css";
// const TAILWIND_CSS: &str = "/assets/tailwind.css";
// const FAVICON: &str = "/assets/favicon.ico";

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(App);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(AppState::default()));

    rsx! {
        // Use runtime cfg! macro instead of #[cfg]
        // if cfg!(target_arch = "wasm32") {
        //     document::Link {
        //         rel: "icon",
        //         href: FAVICON,
        //     }

        //     document::Link {
        //         rel: "stylesheet",
        //         href: TAILWIND_CSS,
        //     }

        //     document::Link {
        //         rel: "stylesheet",
        //         href: MAIN_CSS,
        //     }
        // }

        Router::<Route> {}
    }
}
