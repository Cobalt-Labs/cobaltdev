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
        _ => {
            return Err((StatusCode::UNAUTHORIZED, "Missing or invalid Authorization header".into()));
        }
    };

    let config = crate::config::config::Config::load()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Config loading error".into()))?;

    let mut validation = Validation::default();
    validation.validate_aud = false;

    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &validation,
    ).map_err(|e| {
        eprintln!("JWT decode error: {:?}", e);
        (StatusCode::UNAUTHORIZED, "Invalid token".into())
    })?;

    req.extensions_mut().insert(claims.claims);

    Ok(next.run(req).await)
}