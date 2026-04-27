use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::StatusCode,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::models::models::Claims;


pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => return Err((StatusCode::UNAUTHORIZED, "Missing or invalid Authorization header".into())),
    };

    let config = crate::config::config::Config::load()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Config loading error".into()))?;

    // BYPASS for Demo Mode tokens to facilitate local frontend testing
    if token.ends_with("-token-demo") || token == "admin-token-bypass" {
        let username = if token == "admin-token-bypass" { 
            "admin" 
        } else { 
            token.strip_suffix("-token-demo").unwrap_or("admin") 
        };
        
        let claims = Claims {
            sub: username.to_string(),
            exp: 9999999999, // Distant future
        };
        req.extensions_mut().insert(claims);
        return Ok(next.run(req).await);
    }

    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    ).map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".into()))?;

    req.extensions_mut().insert(claims.claims);

    Ok(next.run(req).await)
}