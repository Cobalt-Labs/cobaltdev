use dioxus::prelude::*;
use crate::hooks::use_files;
use crate::components::{Navbar, UploadDropzone};

#[component]
pub fn HomePage() -> Element {
    let files = use_files::use_files();

    rsx! {
        div { class: "min-h-screen bg-gradient-to-br from-zinc-950 to-zinc-900 border-t-2 border-emerald-500/20 text-white font-sans selection:bg-emerald-500/30 selection:text-emerald-100",
            Navbar {}

            div { class: "max-w-7xl mx-auto px-8 py-10",

                // Hero header
                div { class: "mb-12",
                    h1 { class: "text-5xl font-bold tracking-tighter", "Your Private Cloud" }
                    p { class: "text-zinc-400 mt-3 text-xl", "Powered by Rust • Running on your laptop HDD" }
                }

                UploadDropzone {}

                // Files list - super safe version
                div { class: "mt-12",
                    h2 { class: "text-2xl font-semibold mb-6 flex items-center gap-3",
                        "Recent Files"
                        span { class: "text-sm bg-zinc-800 px-3 py-1 rounded-full text-zinc-400", "{files.read().len()} total" }
                    }

                    if files.read().is_empty() {
                        div { class: "bg-zinc-900 rounded-3xl p-16 text-center",
                            p { class: "text-2xl text-zinc-500", "No files yet" }
                            p { class: "text-zinc-600 mt-2", "Drop a file above to get started" }
                        }
                    } else {
                        div { class: "grid gap-4",
                            for file in files.read().iter() {
                                div { class: "bg-zinc-900/60 backdrop-blur-xl border border-white/5 shadow-lg hover:shadow-[0_8px_30px_rgb(0,0,0,0.5)] hover:-translate-y-1 hover:border-emerald-500/30 transition-all duration-300 p-6 rounded-3xl flex justify-between items-center group",
                                    div {
                                        p { class: "font-medium text-lg", "{file.filename}" }
                                        p { class: "text-xs text-zinc-500 mt-1", "Owner: {file.owner_username} • {file.uploaded_at.format(\"%d %b %Y\")}" }
                                    }
                                    div { class: "text-right font-mono text-emerald-400",
                                        "{((file.size_bytes as f64) / 1_048_576.0).round()} MB"
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