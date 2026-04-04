use dioxus::prelude::*;
// use dioxus_router::prelude::*;

mod models;
mod services;
mod hooks;
mod components;
mod pages;

use pages::{home::HomePage, login::LoginPage};

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    HomePage,
    #[route("/login")]
    LoginPage,
}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        document::Stylesheet {
            href: TAILWIND_CSS
        }
        Router::<Route> {}
    }
}