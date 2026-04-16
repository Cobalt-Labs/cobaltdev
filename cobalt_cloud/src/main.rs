use dioxus::prelude::*;
// use dioxus_router::prelude::*;

/// Tasks for upcoming days..
/// enhance drag and drop feature and add upload from Finder for Cloud-GUI
/// build, train, analyze the backend for the llm model 
/// create an initial setup for the bootloader and kernel

mod models;
mod services;
mod hooks;
mod components;
mod pages;
// ignore line
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
