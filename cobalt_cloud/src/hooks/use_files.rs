use dioxus::prelude::*;
use crate::services::api;
use crate::hooks::use_auth;   // if you have auth
use crate::models::FileMetadata;

pub fn use_files() -> Signal<Vec<FileMetadata>> {   // ← pub fn
    let mut files = use_signal(Vec::new);

    use_effect(move || {
        spawn(async move {
            if let Ok(data) = api::list_files(None).await {   // None for now, later add token
                files.set(data);
            }
        });
    });

    files
}