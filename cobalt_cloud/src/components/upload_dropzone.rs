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

        // evt.data().files() returns Vec<FileData>
        let files = evt.data().files();
        
        for file in files {
            // In Dioxus Desktop, FileData has a path() method
            // We convert the PathBuf to a String for your upload_file function
            let file_path = file.path().to_string_lossy().to_string();

            let mut prog = progress;
            let mut up = uploading;
            let mut st = status;

            spawn(async move {
                up.set(true);
                prog.set(10);
                st.set("Uploading to HDD...".to_string());

                match crate::services::api::upload_file(file_path).await {
                    Ok(_) => {
                        prog.set(100);
                        st.set("✅ File saved to your cloud storage!".to_string());
                    }
                    Err(e) => {
                        prog.set(0);
                        st.set(format!("❌ Failed: {}", e));
                    }
                }
                up.set(false);
            });
        }
    };

    rsx! {
        div {
            class: "border-2 border-dashed rounded-3xl p-12 text-center transition-all",
            // FIX: Using signal() syntax instead of *signal.get()
            class: if is_dragging() { "border-emerald-400 bg-zinc-900/70" } else { "border-zinc-700" },

            ondragenter: ondragenter,
            ondragleave: ondragleave,
            // Required: Browsers won't fire ondrop unless ondragover prevents default
            ondragover: move |e| e.prevent_default(),
            ondrop: ondrop,

            if uploading() {
                div {
                    p { class: "text-lg text-white mb-3", "{status}" }
                    div { class: "w-full bg-zinc-800 h-3 rounded-full overflow-hidden",
                        div {
                            class: "h-full bg-emerald-500 transition-all",
                            // FIX: Direct signal interpolation in attributes
                            style: "width: {progress}%"
                        }
                    }
                }
            } else {
                div {
                    p { class: "text-6xl mb-6", "⬆️" }
                    p { class: "text-2xl font-semibold text-white mb-2", "Drop files here" }
                    p { class: "text-zinc-500", "Files will be saved to your HDD" }
                }
            }
        }
    }
}