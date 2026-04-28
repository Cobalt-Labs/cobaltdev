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
        div { class: "min-h-screen bg-zinc-950 text-white font-sans selection:bg-emerald-500/30 selection:text-emerald-100",
            // Background Decorative Elements
            div { class: "fixed inset-0 overflow-hidden pointer-events-none",
                div { class: "absolute top-0 left-1/4 w-[500px] h-[500px] bg-emerald-500/5 rounded-full blur-[120px] animate-pulse" }
                div { class: "absolute bottom-0 right-1/4 w-[400px] h-[400px] bg-blue-500/5 rounded-full blur-[100px]" }
            }
 
            div { class: "relative z-10",
                Navbar {}
 
                div { class: "max-w-6xl mx-auto px-6 py-12",
 
                    // Hero Section
                    div { class: "mb-16 text-center md:text-left",
                        div { class: "inline-block px-4 py-1.5 bg-emerald-500/10 border border-emerald-500/20 rounded-full text-emerald-400 text-xs font-bold uppercase tracking-widest mb-6 animate-in fade-in slide-in-from-bottom-2", 
                            "Private Ecosystem" 
                        }
                        h1 { class: "text-6xl font-black tracking-tighter mb-4 bg-clip-text text-transparent bg-gradient-to-r from-white via-white to-zinc-500", 
                            "Your Personal Cloud" 
                        }
                        p { class: "text-zinc-400 text-xl max-w-2xl font-medium leading-relaxed", 
                            "End-to-end encrypted storage running directly on your local hardware. No middleman, no subscription, total control." 
                        }
                    }
 
                    // Main Content Grid
                    div { class: "grid lg:grid-cols-3 gap-10",
                        
                        // Left Column: Upload
                        div { class: "lg:col-span-1 space-y-8",
                            div { class: "sticky top-28 space-y-8",
                                // Upload Zone
                                div { class: "bg-zinc-900/40 backdrop-blur-3xl border border-white/5 rounded-[2.5rem] p-8 shadow-2xl",
                                    h3 { class: "text-lg font-bold mb-6 flex items-center gap-3",
                                        span { class: "w-8 h-8 bg-emerald-500/20 rounded-lg flex items-center justify-center text-emerald-400 text-sm", "↑" }
                                        "Quick Upload"
                                    }
                                    UploadDropzone {}
                                    p { class: "text-zinc-500 text-[10px] mt-6 uppercase tracking-widest font-black text-center opacity-50", "Secure Local Transfer" }
                                }
                            }
                        }
 
                        // Right Column: File Explorer
                        div { class: "lg:col-span-2",
                            div { class: "flex items-center justify-between mb-8",
                                h2 { class: "text-2xl font-black tracking-tight flex items-center gap-4",
                                    "File Explorer"
                                    span { class: "text-[10px] bg-zinc-800 text-zinc-400 px-3 py-1 rounded-full uppercase tracking-widest font-black", 
                                        "{filtered_files.read().len()} Objects" 
                                    }
                                }
                                div { class: "flex gap-2",
                                    div { class: "relative group",
                                        input { 
                                            class: "bg-zinc-900 border border-white/5 rounded-xl px-4 py-2 text-sm focus:outline-none focus:border-emerald-500/50 transition-all w-48 group-hover:w-64",
                                            placeholder: "Search vault...",
                                            value: "{search_query}",
                                            oninput: move |e| search_query.set(e.value())
                                        }
                                        span { class: "absolute right-3 top-2.5 text-zinc-500 pointer-events-none", "🔍" }
                                    }
                                    button { 
                                        class: "p-2 bg-zinc-900 rounded-lg border border-white/5 text-zinc-400 hover:text-white transition",
                                        onclick: move |_| {
                                            let mut fs = files_state;
                                            fs.refresh();
                                        },
                                        "🔄" 
                                    }
                                }
                            }
 
                            // Active Uploads Section
                            if !files_state.pending_uploads.read().is_empty() {
                                div { class: "mb-10 space-y-4",
                                    h3 { class: "text-xs font-black text-emerald-500 uppercase tracking-[0.2em] ml-2", "Active Uploads" }
                                    for upload in files_state.pending_uploads.read().iter() {
                                        div { 
                                            key: "{upload.id}",
                                            class: "bg-emerald-500/5 border border-emerald-500/20 rounded-3xl p-6 animate-in fade-in slide-in-from-top-2",
                                            div { class: "flex items-center justify-between mb-3",
                                                div { class: "flex items-center gap-3",
                                                    span { class: "text-xl", "⏳" }
                                                    p { class: "font-bold text-white", "{upload.filename}" }
                                                }
                                                span { class: "text-emerald-400 font-mono font-bold", "{upload.progress}%" }
                                            }
                                            div { class: "h-2 bg-zinc-950 rounded-full overflow-hidden border border-white/5",
                                                div { 
                                                    class: "h-full bg-gradient-to-r from-emerald-500 to-teal-400 transition-all duration-300",
                                                    style: "width: {upload.progress}%"
                                                }
                                            }
                                            p { class: "text-[10px] text-zinc-500 mt-2 font-medium italic", "{upload.status}" }
                                        }
                                    }
                                }
                            }
 
                            if filtered_files.read().is_empty() {
                                div { class: "bg-zinc-900/20 border-2 border-dashed border-white/5 rounded-[2.5rem] py-32 text-center group hover:border-emerald-500/20 transition-all duration-500",
                                    div { class: "w-20 h-20 bg-zinc-900 rounded-3xl flex items-center justify-center mx-auto mb-6 border border-white/5 group-hover:scale-110 transition-transform duration-500",
                                        span { class: "text-4xl grayscale opacity-50 group-hover:grayscale-0 group-hover:opacity-100 transition-all", "📁" }
                                    }
                                    p { class: "text-xl font-bold text-zinc-400", "Cloud is empty" }
                                    p { class: "text-zinc-600 mt-2 font-medium", "Start by dropping files in the upload zone" }
                                }
                            } else {
                                div { class: "grid gap-4",
                                    for file in filtered_files.read().iter() {
                                        div { 
                                            key: "{file.filename}",
                                            class: "group bg-zinc-900/40 backdrop-blur-2xl border border-white/5 p-6 rounded-3xl flex justify-between items-center hover:bg-zinc-800/40 hover:border-emerald-500/30 hover:-translate-y-1 transition-all duration-300 shadow-lg hover:shadow-emerald-500/5",
                                            div { class: "flex items-center gap-5",
                                                div { class: "w-12 h-12 bg-zinc-950 rounded-2xl flex items-center justify-center border border-white/5 group-hover:border-emerald-500/50 transition-colors",
                                                    span { class: "text-xl", "📄" }
                                                }
                                                div {
                                                    p { class: "font-bold text-lg group-hover:text-emerald-400 transition-colors", "{file.filename}" }
                                                    p { class: "text-xs text-zinc-500 font-medium", 
                                                        "{file.owner_username} • {file.uploaded_at.format(\"%b %d, %Y\")}" 
                                                    }
                                                }
                                            }
                                            div { class: "flex items-center gap-6",
                                                div { class: "text-right font-mono text-emerald-500/80 font-bold bg-emerald-500/5 px-3 py-1 rounded-lg border border-emerald-500/10",
                                                    "{((file.size_bytes as f64) / 1_048_576.0).round()} MB"
                                                }
                                                button { class: "w-10 h-10 bg-zinc-950 rounded-xl flex items-center justify-center border border-white/5 text-zinc-400 hover:text-white hover:bg-emerald-500 transition-all opacity-0 group-hover:opacity-100", 
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
}