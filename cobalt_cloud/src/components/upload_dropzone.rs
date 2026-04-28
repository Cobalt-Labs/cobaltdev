use dioxus::html::{FileData, HasFileData};
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn UploadDropzone() -> Element {
    let mut is_dragging = use_signal(|| false);
    let progress = use_signal(|| 0u8);
    let uploading = use_signal(|| false);
    let status = use_signal(|| String::new());
    let files_state = crate::hooks::use_files::use_files_state();

    let auth = crate::hooks::use_auth::use_auth_state();
    let process_files = move |files: Vec<FileData>| {
        for file in files {
            let mut prog = progress;
            let mut up = uploading;
            let mut st = status;
            let mut files_state = files_state;
            let token = auth.peek().token.clone();

            let original_name = file.name();
            let file_name = std::path::Path::new(&original_name)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or(original_name);
            
            let upload_id = Uuid::new_v4().to_string();
            
            // Add to pending state synchronously before spawning async task
            // This ensures the UI updates INSTANTLY
            files_state.add_pending(crate::hooks::use_files::PendingUpload {
                id: upload_id.clone(),
                filename: file_name.clone(),
                progress: 5,
                status: "Initializing...".to_string(),
            });

            spawn(async move {
                up.set(true);
                prog.set(5);
                st.set(format!("Opening connection for {}...", file_name));

                if let Ok(bytes) = file.read_bytes().await {
                    prog.set(40);
                    st.set(format!("Uploading {} ({} bytes)...", file_name, bytes.len()));
                    files_state.update_pending(&upload_id, 40, format!("Uploading ({} bytes)", bytes.len()));

                    match crate::services::api::upload_file_bytes(file_name.clone(), bytes.to_vec(), token).await {
                        Ok(_) => {
                            prog.set(100);
                            st.set("File saved.".to_string());
                            files_state.update_pending(&upload_id, 100, "Done".to_string());
                            files_state.refresh(); // Trigger immediate list refresh
                        }
                        Err(e) => {
                            prog.set(0);
                            st.set(format!("Error: {}", e));
                            files_state.update_pending(&upload_id, 0, format!("Error: {}", e));
                        }
                    }
                } else {
                    prog.set(0);
                    st.set("Could not read file.".to_string());
                    files_state.update_pending(&upload_id, 0, "Read error".to_string());
                }

                if prog() == 100 {
                    up.set(false);
                    prog.set(0);
                    st.set(String::new());
                    // Delay removal slightly so user sees success
                    files_state.remove_pending(&upload_id);
                } else {
                    up.set(false);
                }
            });
        }
    };

    let ondragenter = move |evt: DragEvent| {
        evt.prevent_default();
        is_dragging.set(true);
    };
    let ondragleave = move |evt: DragEvent| {
        evt.prevent_default();
        is_dragging.set(false);
    };
    let ondragover = move |evt: DragEvent| {
        evt.prevent_default();
    };

    let ondrop = move |evt: DragEvent| {
        evt.prevent_default();
        is_dragging.set(false);
        process_files(evt.files());
    };

    let onchange = move |evt: FormEvent| {
        evt.prevent_default();
        process_files(evt.files());
    };

    rsx! {
        label {
            class: "block w-full border-2 border-dashed rounded-xl p-10 text-center transition-colors duration-200 cursor-pointer",
            class: if is_dragging() { "border-blue-500 bg-blue-500/5" } else { "border-zinc-700 hover:border-zinc-500 hover:bg-zinc-800/50" },
            
            ondragenter: ondragenter,
            ondragleave: ondragleave,
            ondragover: ondragover,
            ondrop: ondrop,

            input {
                r#type: "file",
                multiple: true,
                accept: "*/*",
                style: "display: none;",
                onchange: onchange,
            }

            if uploading() {
                div { class: "py-2 pointer-events-none",
                    p { class: "text-sm text-zinc-300 mb-3", "{status}" }
                    div { class: "w-full h-1.5 bg-zinc-700 rounded-full overflow-hidden",
                        div {
                            class: "h-full bg-blue-500 rounded-full transition-all duration-500",
                            style: "width: {progress}%",
                        }
                    }
                }
            } else {
                div { class: "pointer-events-none",
                    p {
                        class: "text-2xl mb-3",
                        if is_dragging() { "📂" } else { "↑" }
                    }
                    p { class: "text-sm font-medium text-zinc-300 mb-1",
                        if is_dragging() { "Drop to upload" } else { "Drop files here" }
                    }
                    p { class: "text-xs text-zinc-500", "or click to browse" }
                }
            }
        }
    }
}
