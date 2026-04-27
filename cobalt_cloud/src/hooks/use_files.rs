use dioxus::prelude::*;
use crate::services::api;
use crate::models::FileMetadata;

#[derive(Clone, Copy)]
pub struct FilesState {
    pub files: Signal<Vec<FileMetadata>>,
    pub refresh: Signal<usize>, // Use a counter to trigger re-fetches
}

impl FilesState {
    pub fn refresh(&mut self) {
        let next_val = *self.refresh.peek() + 1;
        self.refresh.set(next_val);
    }
}

pub fn use_provide_files_context(auth: Signal<crate::hooks::use_auth::AuthState>) -> FilesState {
    let files = use_signal(Vec::new);
    let refresh = use_signal(|| 0);
    let state = FilesState { files, refresh };
    
    use_context_provider(|| state);

    use_effect(move || {
        let mut files = files;
        let _ = refresh(); // Track refresh counter
        let auth_val = auth.read();
        let token = auth_val.token.clone();
        
        spawn(async move {
            if let Ok(data) = api::list_files(token).await {
                files.set(data);
            }
        });
    });

    state
}

pub fn use_files_state() -> FilesState {
    use_context::<FilesState>()
}

pub fn use_files() -> Signal<Vec<FileMetadata>> {
    use_files_state().files
}