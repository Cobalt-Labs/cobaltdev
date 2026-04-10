use dioxus::prelude::*;
use std::time::Duration;
use crate::{api, Route};

#[component]
pub fn Signup() -> Element {
    let nav = navigator();
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut confirm_password = use_signal(|| String::new());
    let error_msg = use_signal(|| String::new());
    let success_msg = use_signal(|| String::new());
    let loading = use_signal(|| false);

    let handle_signup = {
        let username = username.clone();
        let password = password.clone();
        let confirm_password = confirm_password.clone();
        let mut error_msg = error_msg.clone();
        let mut success_msg = success_msg.clone();
        let mut loading = loading.clone();
        let nav = nav.clone();

        move |_| {
            if password() != confirm_password() {
                error_msg.set("Passwords do not match".to_string());
                return;
            }

            spawn({
                let username_val = username();
                let password_val = password();
                async move {
                    loading.set(true);
                    error_msg.set(String::new());
                    success_msg.set(String::new());

                    match api::signup(username_val, password_val).await {
                        Ok(auth_resp) => {
                            if auth_resp.ok {
                                success_msg.set("Account created! Redirecting to login...".to_string());
                                #[cfg(target_arch = "wasm32")]
                                gloo_timers::future::sleep(Duration::from_secs(1)).await;
                                #[cfg(not(target_arch = "wasm32"))]
                                async_std::task::sleep(Duration::from_secs(1)).await;
                                nav.push(Route::Login {});
                            } else {
                                error_msg.set(auth_resp.message);
                            }
                        }
                        Err(e) => {
                            error_msg.set(e);
                        }
                    }

                    loading.set(false);
                }
            });
        }
    };

    rsx! {
        div {
            class: "min-h-screen bg-gradient-to-br from-purple-50 to-pink-100 flex items-center justify-center p-4",
            
            div {
                class: "bg-white rounded-2xl shadow-2xl p-8 w-full max-w-md",
                
                div {
                    class: "text-center mb-8",
                    h1 { class: "text-4xl font-bold text-gray-800 mb-2", "Create Account" }
                    p { class: "text-gray-600", "Start your journaling journey today" }
                }

                if !error_msg().is_empty() {
                    div {
                        class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded-lg mb-4",
                        "{error_msg()}"
                    }
                }

                if !success_msg().is_empty() {
                    div {
                        class: "bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded-lg mb-4",
                        "{success_msg()}"
                    }
                }

                div {
                    class: "space-y-4",

                    div {
                        label { class: "block text-sm font-medium text-gray-700 mb-2", "Username" }
                        input {
                            class: "w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent text-gray-900 transition",
                            r#type: "text",
                            placeholder: "Choose a username",
                            value: "{username()}",
                            oninput: move |e| username.set(e.value().clone()),
                        }
                    }

                    div {
                        label { class: "block text-sm font-medium text-gray-700 mb-2", "Password" }
                        input {
                            class: "w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent text-gray-900 transition",
                            r#type: "password",
                            placeholder: "Choose a password",
                            value: "{password()}",
                            oninput: move |e| password.set(e.value().clone()),
                        }
                    }

                    div {
                        label { class: "block text-sm font-medium text-gray-700 mb-2", "Confirm Password" }
                        input {
                            class: "w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent text-gray-900 transition",
                            r#type: "password",
                            placeholder: "Confirm your password",
                            value: "{confirm_password()}",
                            oninput: move |e| confirm_password.set(e.value().clone()),
                        }
                    }

                    button {
                        class: "w-full bg-purple-600 hover:bg-purple-700 text-white font-semibold py-3 px-4 rounded-lg transition duration-200 transform hover:scale-105",
                        disabled: loading(),
                        onclick: handle_signup,
                        if loading() {
                            "Creating account..."
                        } else {
                            "Sign Up"
                        }
                    }

                    div {
                        class: "text-center mt-4",
                        span { class: "text-gray-600", "Already have an account? " }
                        Link {
                            to: Route::Login {},
                            class: "text-purple-600 hover:text-purple-800 font-semibold",
                            "Login"
                        }
                    }
                }
            }
        }
    }
}
