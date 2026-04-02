#![allow(non_snake_case)]
use dioxus::prelude::*;
use futures_timer::Delay;
use std::time::Duration;

#[component]
pub fn UploadDropzone() -> Element {
    let mut is_dragging = use_signal(|| false);
    let mut progress = use_signal(|| 0u8);
    let mut uploading = use_signal(|| false);

    let ondragenter = move |_| is_dragging.set(true);
    let ondragleave = move |_| is_dragging.set(false);

    // FIX: Ensure the closure explicitly returns unit ()
    let ondrop = move |_evt: DragEvent| {
        is_dragging.set(false);
        uploading.set(true);
        progress.set(0);

        spawn(async move {
            for i in 1..=100 {
                Delay::new(Duration::from_millis(40)).await;
                progress.set(i);
            }
            uploading.set(false);
            progress.set(0);
        });

        // This line is the magic fix for the "found ()" error
        () 
    };

    rsx! {
        div {
            class: "border-2 border-dashed rounded-3xl p-12 text-center transition-all cursor-pointer",
            class: if is_dragging() { "border-emerald-400 bg-zinc-900/70" } else { "border-zinc-700" },
            
            ondragenter: ondragenter,
            ondragleave: ondragleave,
            ondragover: move |e| e.stop_propagation(), 
            ondrop: ondrop,

            if uploading() {
                div {
                    p { class: "text-lg text-white", "Uploading to your Rust HDD Cloud..." }
                    div { class: "w-full bg-zinc-800 h-3 rounded-full mt-4 overflow-hidden",
                        div {
                            class: "h-full bg-gradient-to-r from-emerald-400 to-teal-400 transition-all duration-200",
                            style: "width: {progress}%"
                        }
                    }
                    p { class: "text-xs font-mono text-zinc-400 mt-2", "{progress}% complete" }
                }
            } else {
                div {
                    p { class: "text-6xl mb-6", "⬆️" }
                    p { class: "text-2xl font-semibold text-white mb-2", "Drop files here" }
                    p { class: "text-zinc-500", "or click to browse" }
                }
            }
        }
    }
}