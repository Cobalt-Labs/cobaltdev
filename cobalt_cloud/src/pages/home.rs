use dioxus::prelude::*;
use crate::hooks::use_files;
use crate::components::{Navbar, UploadDropzone};

#[component]
pub fn HomePage() -> Element {
    let files = use_files::use_files();
    let files_state = crate::hooks::use_files::use_files_state();
    let mut search_query = use_signal(String::new);

    let filtered_files = use_memo(move || {
        let query = search_query().to_lowercase();
        files.read()
            .iter()
            .filter(|f| f.filename.to_lowercase().contains(&query))
            .cloned()
            .collect::<Vec<_>>()
    });

    rsx! {
        div { class: "min-h-screen bg-zinc-900 text-white",

            Navbar {}

            div { class: "max-w-5xl mx-auto px-6 py-10",

                // Page header
                div { class: "mb-8",
                    h1 { class: "text-xl font-semibold text-white", "Files" }
                    p { class: "text-sm text-zinc-400 mt-1", "Your private local storage" }
                }

                div { class: "grid grid-cols-1 lg:grid-cols-3 gap-6",

                    // Left: Upload
                    div { class: "lg:col-span-1",
                        div { class: "bg-zinc-800 border border-zinc-700 rounded-xl p-5",
                            p { class: "text-sm font-medium text-zinc-300 mb-4", "Upload" }
                            UploadDropzone {}
                        }
                    }

                    // Right: File list
                    div { class: "lg:col-span-2",

                        // Toolbar
                        div { class: "flex items-center justify-between mb-4",
                            span { class: "text-sm text-zinc-400",
                                "{filtered_files.read().len()} file(s)"
                            }
                            div { class: "flex items-center gap-2",
                                input {
                                    class: "bg-zinc-800 border border-zinc-700 rounded-lg px-3 py-1.5 text-sm text-white placeholder-zinc-500 focus:outline-none focus:border-zinc-500 transition-colors",
                                    placeholder: "Search...",
                                    value: "{search_query}",
                                    oninput: move |e| search_query.set(e.value())
                                }
                                button {
                                    class: "text-zinc-400 hover:text-white text-sm px-2 py-1.5 rounded-lg hover:bg-zinc-800 transition-colors",
                                    onclick: move |_| {
                                        let mut fs = files_state;
                                        fs.refresh();
                                    },
                                    "Refresh"
                                }
                            }
                        }

                        // Active Uploads
                        if !files_state.pending_uploads.read().is_empty() {
                            div { class: "mb-4 space-y-2",
                                for upload in files_state.pending_uploads.read().iter() {
                                    div {
                                        key: "{upload.id}",
                                        class: "bg-zinc-800 border border-zinc-700 rounded-xl p-4",
                                        div { class: "flex items-center justify-between mb-2",
                                            p { class: "text-sm font-medium text-zinc-200 truncate", "{upload.filename}" }
                                            span { class: "text-xs text-zinc-400 ml-2 shrink-0", "{upload.progress}%" }
                                        }
                                        div { class: "h-1 bg-zinc-700 rounded-full overflow-hidden",
                                            div {
                                                class: "h-full bg-blue-500 rounded-full transition-all duration-300",
                                                style: "width: {upload.progress}%"
                                            }
                                        }
                                        p { class: "text-xs text-zinc-500 mt-1.5", "{upload.status}" }
                                    }
                                }
                            }
                        }

                        // File list
                        if filtered_files.read().is_empty() {
                            div { class: "bg-zinc-800 border border-zinc-700 rounded-xl p-12 text-center",
                                p { class: "text-zinc-500 text-sm", "No files yet" }
                                p { class: "text-zinc-600 text-xs mt-1", "Upload something to get started" }
                            }
                        } else {
                            div { class: "bg-zinc-800 border border-zinc-700 rounded-xl divide-y divide-zinc-700 overflow-hidden",
                                for file in filtered_files.read().iter() {
                                    div {
                                        key: "{file.id}",
                                        class: "flex items-center justify-between px-4 py-3 hover:bg-zinc-750 transition-colors group",
                                        div { class: "flex items-center gap-3 min-w-0",
                                            span { class: "text-zinc-500 shrink-0", "📄" }
                                            div { class: "min-w-0",
                                                p { class: "text-sm text-white font-medium truncate", "{file.filename}" }
                                                p { class: "text-xs text-zinc-500 truncate",
                                                    "{file.owner_username} · {file.uploaded_at}"
                                                }
                                            }
                                        }
                                        div { class: "flex items-center gap-3 shrink-0 ml-4",
                                            span { class: "text-xs text-zinc-400 font-mono",
                                                "{((file.size_bytes as f64) / 1_048_576.0 * 100.0).round() / 100.0} MB"
                                            }
                                            button {
                                                class: "text-xs text-zinc-500 hover:text-white transition-colors opacity-0 group-hover:opacity-100",
                                                "↓"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}