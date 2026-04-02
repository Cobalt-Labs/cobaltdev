use dioxus::prelude::*;
use crate::hooks::use_files;
use crate::models::FileMetadata;
use use_files::use_files;

#[component]
pub fn HomePage() -> Element {
    let files = use_files();

    rsx! {
        div { class: "min-h-screen bg-zinc-950 text-white p-8",
            div { class: "max-w-6xl mx-auto",
                // Header
                div { class: "flex justify-between items-center mb-10",
                    h1 { class: "text-4xl font-bold tracking-tight", "Cobalt Cloud" }
                    p { class: "text-zinc-400", "Your Room • Pure Rust HDD Cloud 🔥" }
                }

                // Quick stats
                div { class: "grid grid-cols-3 gap-6 mb-10",
                    div { class: "bg-zinc-900 p-6 rounded-2xl",
                        p { class: "text-zinc-400 text-sm", "Total Files" }
                        p { class: "text-5xl font-mono font-bold mt-2", "{files.read().len()}" }
                    }
                    div { class: "bg-zinc-900 p-6 rounded-2xl",
                        p { class: "text-zinc-400 text-sm", "Used Space" }
                        p { class: "text-5xl font-mono font-bold mt-2", "2.4 GB" }
                    }
                    div { class: "bg-zinc-900 p-6 rounded-2xl",
                        p { class: "text-zinc-400 text-sm", "Connected Backend" }
                        p { class: "text-emerald-400 text-2xl font-medium mt-2", "● Live" }
                    }
                }

                // Files section
                div { class: "bg-zinc-900 rounded-3xl p-8",
                    h2 { class: "text-2xl font-semibold mb-6", "Your Files" }

                    if files.read().is_empty() {
                        div { class: "text-center py-20 text-zinc-500",
                            p { "No files yet. Upload something!" }
                        }
                    } else {
                        div { class: "space-y-3",
                            for file in files.read().iter() {
                                div { class: "flex items-center justify-between bg-zinc-800 hover:bg-zinc-700 transition-colors p-5 rounded-2xl",
                                    div {
                                        p { class: "font-medium", "{file.filename}" }
                                        p { class: "text-xs text-zinc-500 mt-1", "Uploaded • {file.uploaded_at.format(\"%b %d\")}" }
                                    }
                                    div { class: "text-right",
                                        p { class: "text-sm text-emerald-400 font-mono", "{(file.size_bytes as f64 / 1_048_576.0).round()} MB" }
                                    }
                                }
                            }
                        }
                    }
                }

                // Quick upload button (we'll make it drag-drop later)
                button {
                    class: "mt-8 w-full bg-white text-black font-semibold py-4 rounded-2xl hover:bg-zinc-200 transition",
                    onclick: move |_| { /* open upload later */ },
                    "↑ Upload New File to Your Cloud"
                }
            }
        }
    }
}