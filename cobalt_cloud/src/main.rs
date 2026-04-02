use dioxus::prelude::*;
// use dioxus_router::prelude::*;
use dioxus_router::{Routable, Router};
use pages::home::HomePage;

mod models;
mod services;
mod hooks;
mod pages;

fn main() {
    dioxus::launch(app);
}

#[derive(Routable, Clone, PartialEq, Debug)]
enum Route {
    #[route("/")]
    HomePage,
}

fn app() -> Element {
    rsx! {
        Router::<Route> {}
    }
}