//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component and an Echo component for fullstack apps to be used in our app.

#[allow(dead_code)]
mod hero;
#[allow(unused_imports)]
pub use hero::Hero;

#[allow(dead_code)]
mod echo;
#[allow(unused_imports)]
pub use echo::Echo;

mod navbar;
pub use navbar::Navbar;

mod upload_dropzone;
pub use upload_dropzone::UploadDropzone;