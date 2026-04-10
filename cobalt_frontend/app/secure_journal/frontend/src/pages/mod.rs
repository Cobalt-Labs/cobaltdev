use dioxus::prelude::*;
use dioxus::router::Routable;

pub mod entries;
pub mod home;
pub mod login;
pub mod new_entry;
pub mod signup;

pub use entries::Entries;
pub use home::Home;
pub use login::Login;
pub use new_entry::NewEntry;
pub use signup::Signup;

#[derive(Routable, Clone)]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/login")]
    Login {},

    #[route("/signup")]
    Signup {},

    #[route("/entries")]
    Entries {},

    #[route("/new")]
    NewEntry {},
}
