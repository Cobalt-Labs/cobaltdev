use dioxus::html::HasFileData;
use dioxus::prelude::*;

#[component]
pub fn UploadDropzone() -> Element {
    let mut is_dragging = use_signal(|| false);
    let progress = use_signal(|| 0u8);
    let uploading = use_signal(|| false);
    let status = use_signal(|| String::new());

    let ondragenter = move |_| is_dragging.set(true);
    let ondragleave = move |_| is_dragging.set(false);

    let ondrop = move |evt: DragEvent| {
        evt.prevent_default();
        is_dragging.set(false);

        // Access files from the event (returns Vec<FileData> in Dioxus 0.7)
        let files = evt.files();

        for file in files {
            let mut prog = progress;
            let mut up = uploading;
            let mut st = status;

            // Capture name from file
            let file_name = file.name();

            spawn(async move {
                up.set(true);
                prog.set(10);
                st.set(format!("Reading {}...", file_name));

                if let Ok(bytes) = file.read_bytes().await {
                    prog.set(50);
                    st.set("Uploading...".to_string());

                    match crate::services::api::upload_file_bytes(file_name, bytes.to_vec()).await {
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
                    st.set("❌ Could not read file content".to_string());
                }

                if prog() == 100 {
                    // Delay for better UX before clearing status (Desktop only for tokio::time safety)
                    #[cfg(not(target_arch = "wasm32"))]
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
