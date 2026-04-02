use dioxus::prelude::*;
use crate::services::api;
use crate::models::FileMetadata;

pub fn use_files() -> Signal<Vec<FileMetadata>> {
    let mut files = use_signal(|| vec![]);

    use_future(move || async move {
        if let Ok(data) = api::list_files().await {
            files.set(data);
        }
    });

    files
}