use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::{HeaderValue, header},
    Router,
};
use tower_http::cors::{CorsLayer, AllowOrigin};

pub fn add_security_layers<S>(app: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(move |origin: &HeaderValue, _parts: &axum::http::request::Parts| {
            if let Ok(origin_str) = std::str::from_utf8(origin.as_bytes()) {
                origin_str.starts_with("http://localhost:")
                    || origin_str.starts_with("http://127.0.0.1:")
                    || origin_str == "https://cobaltdev.vercel.app"
                    || origin_str == "http://localhost:5173"
                    || origin_str == "http://localhost:3000"
                    || origin_str == "http://localhost:8080"
            } else {
                false
            }
        }))
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
        ])
        .allow_credentials(true);

    app.layer(cors)
        .layer(axum::middleware::from_fn(security_headers_middleware))
}

pub async fn security_headers_middleware(
    request: Request,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();

    headers.insert(
        "X-Frame-Options",
        HeaderValue::from_static("DENY"),
    );
    
    headers.insert(
        "X-Content-Type-Options",
        HeaderValue::from_static("nosniff"),
    );
    
    headers.insert(
        "Strict-Transport-Security",
        HeaderValue::from_static("max-age=31536000; includeSubDomains"),
    );
    
    headers.insert(
        "Referrer-Policy",
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    
    headers.insert(
        "Content-Security-Policy",
        HeaderValue::from_static("default-src 'self' 'unsafe-inline' 'unsafe-eval' http://localhost:* http://127.0.0.1:* ws://localhost:* ws://127.0.0.1:* data: blob:"),
    );

    response
}