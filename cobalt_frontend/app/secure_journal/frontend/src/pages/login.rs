use crate::{api, state::AppState, Route};
use dioxus::prelude::*;
// test code git

#[component]
pub fn Login() -> Element {
    let nav = navigator();
    let mut app_state = use_context::<Signal<AppState>>();
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error_msg = use_signal(|| String::new());
    let mut loading = use_signal(|| false);
    let mut show_password = use_signal(|| false);

    let handle_login = move |_| {
        let username_val = username();
        let password_val = password();

        spawn(async move {
            loading.set(true);
            error_msg.set(String::new());

            match api::login(username_val.clone(), password_val).await {
                Ok(auth_resp) => {
                    if auth_resp.ok {
                        app_state.write().logged_in = true;
                        app_state.write().username = username_val;
                        nav.push(Route::Home {});
                    } else {
                        error_msg.set(auth_resp.message);
                    }
                }
                Err(e) => error_msg.set(e),
            }

            loading.set(false);
        });
    };

    rsx! {
        div {
            class: "min-h-screen flex items-center justify-center
                    bg-gradient-to-br from-indigo-900 via-purple-900 to-black
                    relative overflow-hidden",

            // Animated blurred background glow
            div {
                class: "absolute w-[600px] h-[600px] bg-purple-500/30
                        rounded-full blur-3xl animate-pulse top-[-200px] left-[-200px]"
            }
            div {
                class: "absolute w-[500px] h-[500px] bg-pink-500/30
                        rounded-full blur-3xl animate-pulse bottom-[-200px] right-[-200px]"
            }

            // Glass Card
            div {
                class: "relative bg-white/10 backdrop-blur-2xl
                        border border-white/20 shadow-2xl
                        rounded-3xl p-10 w-full max-w-md
                        transition-all duration-300 hover:scale-[1.02]",

                // Title
                div {
                    class: "text-center mb-8",
                    h1 {
                        class: "text-4xl font-extrabold text-white tracking-wide",
                        "🔐 Secure Journal"
                    }
                    p {
                        class: "text-white/70 mt-2 text-sm",
                        "Private. Encrypted. Yours."
                    }
                }

                // Error
                if !error_msg().is_empty() {
                    div {
                        class: "bg-red-500/20 border border-red-400
                                text-red-200 px-4 py-3 rounded-xl mb-5
                                animate-fade-in",
                        "{error_msg()}"
                    }
                }

                div {
                    class: "space-y-6",

                    // Username Field
                    div {
                        class: "relative",
                        input {
                            class: "peer w-full px-4 pt-5 pb-2 rounded-xl
                                    bg-white/20 text-white border border-white/30
                                    placeholder-transparent focus:outline-none
                                    focus:ring-2 focus:ring-purple-400
                                    transition",
                            r#type: "text",
                            placeholder: "Username",
                            value: "{username()}",
                            oninput: move |e| username.set(e.value().clone()),
                        }
                        label {
                            class: "absolute left-4 top-2 text-white/60 text-sm
                                    peer-placeholder-shown:top-4
                                    peer-placeholder-shown:text-base
                                    peer-placeholder-shown:text-white/40
                                    transition-all",
                            "Username"
                        }
                    }

                    // Password Field
                    div {
                        class: "relative",
                        input {
                            class: "peer w-full px-4 pt-5 pb-2 rounded-xl
                                    bg-white/20 text-white border border-white/30
                                    placeholder-transparent focus:outline-none
                                    focus:ring-2 focus:ring-purple-400
                                    transition",
                            r#type: if show_password() { "text" } else { "password" },
                            placeholder: "Password",
                            value: "{password()}",
                            oninput: move |e| password.set(e.value().clone()),
                        }
                        label {
                            class: "absolute left-4 top-2 text-white/60 text-sm
                                    peer-placeholder-shown:top-4
                                    peer-placeholder-shown:text-base
                                    peer-placeholder-shown:text-white/40
                                    transition-all",
                            "Password"
                        }

                        button {
                            class: "absolute right-4 top-4 text-white/70 hover:text-white",
                            onclick: move |_| show_password.set(!show_password()),
                            if show_password() { "🙈" } else { "👁️" }
                        }
                    }

                    // Login Button
                    button {
                        class: "w-full bg-gradient-to-r from-purple-500 to-pink-500
                                hover:from-pink-500 hover:to-purple-500
                                text-white font-bold py-3 rounded-xl
                                transition-all duration-300
                                shadow-lg hover:shadow-pink-500/50
                                transform hover:scale-[1.03]",
                        disabled: loading(),
                        onclick: handle_login,

                        if loading() {
                            span { class: "animate-spin mr-2 inline-block", "⏳" }
                            "Authenticating..."
                        } else {
                            "Login"
                        }
                    }

                    // Divider
                    div {
                        class: "flex items-center my-4",
                        div { class: "flex-grow h-px bg-white/20" }
                        span { class: "px-3 text-white/50 text-sm", "or" }
                        div { class: "flex-grow h-px bg-white/20" }
                    }

                    // Signup
                    div {
                        class: "text-center",
                        span { class: "text-white/70 text-sm", "Don't have an account? " }
                        Link {
                            to: Route::Signup {},
                            class: "text-purple-300 hover:text-pink-300 font-semibold underline transition",
                            "Create Account"
                        }
                    }
                }
            }
        }
    }
}
