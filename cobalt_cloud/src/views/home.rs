use dioxus::prelude::*;
use crate::components::{Hero, Echo};

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Echo {}
    }
}
