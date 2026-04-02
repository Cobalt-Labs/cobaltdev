use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct AuthState {
    pub token: Option<String>,
    pub username: Option<String>,
}

pub fn use_auth() -> Signal<AuthState> {
    use_signal(|| AuthState {
        token: None,
        username: Some("ibrahim3595".to_string()), // default for now
    })
}