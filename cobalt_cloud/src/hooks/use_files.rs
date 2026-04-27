use dioxus::prelude::*;
use crate::services::api;
use crate::models::FileMetadata;

#[derive(Clone, PartialEq)]
pub struct PendingUpload {
    pub id: String,
    pub filename: String,
    pub progress: u8,
    pub status: String,
}

#[derive(Clone, Copy)]
pub struct FilesState {
    pub files: Signal<Vec<FileMetadata>>,
    pub pending_uploads: Signal<Vec<PendingUpload>>,
    pub refresh: Signal<usize>, // Use a counter to trigger re-fetches
}

impl FilesState {
    pub fn refresh(&mut self) {
        let next_val = *self.refresh.peek() + 1;
        self.refresh.set(next_val);
    }

    pub fn add_pending(&mut self, upload: PendingUpload) {
        self.pending_uploads.write().push(upload);
    }

    pub fn update_pending(&mut self, id: &str, progress: u8, status: String) {
        let mut pending = self.pending_uploads.write();
        if let Some(u) = pending.iter_mut().find(|u| u.id == id) {
            u.progress = progress;
            u.status = status;
        }
    }

    pub fn remove_pending(&mut self, id: &str) {
        self.pending_uploads.write().retain(|u| u.id != id);
    }
}

pub fn use_provide_files_context(auth: Signal<crate::hooks::use_auth::AuthState>) -> FilesState {
    let files = use_signal(Vec::new);
    let pending_uploads = use_signal(Vec::new);
    let refresh = use_signal(|| 0);
    let state = FilesState { files, pending_uploads, refresh };
    
    use_context_provider(|| state);

    use_effect(move || {
        let mut files = files;
        let _ = refresh(); // Track refresh counter
        let auth_val = auth.read();
        let token = auth_val.token.clone();
        
        spawn(async move {
            match api::list_files(token).await {
                Ok(data) => files.set(data),
                Err(e) => {
                    eprintln!("Failed to fetch files: {}", e);
                    // We could add an error signal to FilesState if we want to show it in UI
                }
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