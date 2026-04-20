use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct AuthState {
    pub token: Option<String>,
    pub username: Option<String>,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            token: None,
            username: None,
        }
    }
}

pub fn use_provide_auth_context() -> Signal<AuthState> {
    use_context_provider(|| Signal::new(AuthState::default()))
}

pub fn use_auth_state() -> Signal<AuthState> {
    use_context::<Signal<AuthState>>()
}