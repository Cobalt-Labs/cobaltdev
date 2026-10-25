use crate::models::{AuthRequest, AuthResponse};

const API_BASE_URL: &str = "http://127.0.0.1:8000/api";

#[cfg(not(target_arch = "wasm32"))]
use reqwest::Client;

#[cfg(target_arch = "wasm32")]
use gloo_net::http::Request;

pub async fn login(username: String, password: String) -> Result<AuthResponse, String> {

    #[cfg(not(target_arch = "wasm32"))]
    {
        let client = Client::new();

        let response = client
            .post(format!("{}/login", API_BASE_URL))
            .json(&AuthRequest { username, password })
            .send()
            .await
            .map_err(|e| e.to_string())?;

        response
            .json::<AuthResponse>()
            .await
            .map_err(|e| e.to_string())
    }

    #[cfg(target_arch = "wasm32")]
    {
        let response = Request::post(&format!("{}/login", API_BASE_URL))
            .json(&AuthRequest { username, password })
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        response
            .json::<AuthResponse>()
            .await
            .map_err(|e| e.to_string())
    }
}

pub async fn signup(username: String, password: String) -> Result<AuthResponse, String> {

    #[cfg(not(target_arch = "wasm32"))]
    {
        let client = Client::new();

        let response = client
            .post(format!("{}/signup", API_BASE_URL))
            .json(&AuthRequest { username, password })
            .send()
            .await
            .map_err(|e| e.to_string())?;

        response
            .json::<AuthResponse>()
            .await
            .map_err(|e| e.to_string())
    }

    #[cfg(target_arch = "wasm32")]
    {
        let response = Request::post(&format!("{}/signup", API_BASE_URL))
            .json(&AuthRequest { username, password })
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        response
            .json::<AuthResponse>()
            .await
            .map_err(|e| e.to_string())
    }
}
