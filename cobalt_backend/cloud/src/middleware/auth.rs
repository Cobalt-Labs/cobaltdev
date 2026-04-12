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

    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret("your-jwt-secret-from-env".as_ref()),
        &Validation::default(),
    ).map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".into()))?;

    req.extensions_mut().insert(claims.claims);

    Ok(next.run(req).await)
}