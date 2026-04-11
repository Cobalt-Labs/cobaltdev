use dioxus::prelude::*;
use dioxus::html::HasFileData;

#[component]
pub fn UploadDropzone() -> Element {
    let mut is_dragging = use_signal(|| false);
    let mut progress = use_signal(|| 0u8);
    let mut uploading = use_signal(|| false);
    let mut status = use_signal(|| String::new());

    let ondragenter = move |_| is_dragging.set(true);
    let ondragleave = move |_| is_dragging.set(false);

    let ondrop = move |evt: DragEvent| {
        evt.prevent_default();
        is_dragging.set(false);

        // Keep the user's working FileData usage since Dioxus Desktop 0.7 apparently uses it here
        let files = evt.data().files();
        
        for file in files {
            // Need to specify the type to help the compiler in some Edge cases
            let file_path = file.path().to_string_lossy().to_string();
            let mut prog = progress;
            let mut up = uploading;
            let mut st = status;

            spawn(async move {
                up.set(true);
                prog.set(10);
                st.set(format!("Reading {}...", file_path));

                if let Ok(file_bytes) = tokio::fs::read(&file_path).await {
                    let f_name = std::path::Path::new(&file_path)
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();

                    prog.set(50);
                    st.set("Uploading...".to_string());

                    match crate::services::api::upload_file_bytes(f_name, file_bytes).await {
                        Ok(_) => {
                            prog.set(100);
                            st.set("✅ File securely saved!".to_string());
                        }
                        Err(e) => {
                            prog.set(0);
                            st.set(format!("❌ Failed API: {}", e));
                        }
                    }
                } else {
                    prog.set(0);
                    st.set(format!("❌ Could not read file from path"));
                }
                
                if prog() == 100 {
                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                    up.set(false);
                    prog.set(0);
                    st.set(String::new());
                } else {
                    up.set(false);
                }
            });
        }
    };

    rsx! {
        div {
            class: "border-2 border-dashed rounded-3xl p-16 text-center transition-all duration-300 transform",
            class: if is_dragging() { "border-emerald-400 bg-emerald-900/10 shadow-[0_0_50px_-10px_rgba(16,185,129,0.3)] scale-[1.02]" } else { "border-zinc-700/40 hover:border-zinc-500 hover:bg-zinc-900/40" },

            ondragenter: ondragenter,
            ondragleave: ondragleave,
            ondragover: move |e| e.prevent_default(),
            ondrop: ondrop,

            if uploading() {
                div { class: "py-6",
                    p { class: "text-lg text-emerald-100 font-medium tracking-wide mb-4", "{status}" }
                    div { class: "w-full max-w-sm mx-auto bg-zinc-950/80 p-1 h-4 rounded-full overflow-hidden border border-white/5 shadow-inner",
                        div {
                            class: "h-full bg-gradient-to-r from-emerald-500 to-teal-400 rounded-full transition-all duration-500 ease-out relative overflow-hidden",
                            style: "width: {progress}%",
                            div { class: "absolute inset-0 bg-white/20 blur-[2px] right-0 translate-x-1/2 w-8" }
                        }
                    }
                }
            } else {
                div {
                    p { 
                        class: "text-6xl mb-6 transition-transform duration-300 drop-shadow-xl",
                        class: if is_dragging() { "scale-110 -translate-y-2" } else { "text-zinc-600 hover:text-white" },
                        if is_dragging() { "☁️" } else { "⬆️" }
                    }
                    p { class: "text-2xl font-bold text-white mb-2 tracking-tight", "Secure Dropzone" }
                    p { class: "text-zinc-400 font-medium", "Drag and drop files to instantly sync to your HDD" }
                }
            }
        }
    }
}