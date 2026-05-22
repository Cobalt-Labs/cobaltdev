use axum::{
    middleware::Next,
    response::Response,
    http::{Request, HeaderValue, header},
    Router,
};
use tower_http::cors::CorsLayer;

/// Applies production-hardened security layers: strict CORS + security headers.
/// Call this in main.rs on your router: `let app = add_security_layers(app);`
pub fn add_security_layers<S>(app: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    // Strict CORS — only allow known Cobalt origins.
    // TODO: Replace with your actual production domain(s) before deploying.
    let cors = CorsLayer::new()
        .allow_origin([
            "https://cobaltdev.vercel.app"
                .parse::<HeaderValue>()
                .expect("Invalid CORS origin"),
        ])
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
        .allow_credentials(false);

    app.layer(cors)
        .layer(axum::middleware::from_fn(security_headers_middleware))
}

/// Injects standard security headers on every response.
pub async fn security_headers_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();

    // Prevent clickjacking
    headers.insert(
        "X-Frame-Options",
        HeaderValue::from_static("DENY"),
    );
    // Prevent MIME sniffing
    headers.insert(
        "X-Content-Type-Options",
        HeaderValue::from_static("nosniff"),
    );
    // Enforce HTTPS
    headers.insert(
        "Strict-Transport-Security",
        HeaderValue::from_static("max-age=31536000; includeSubDomains"),
    );
    // Restrict referrer info
    headers.insert(
        "Referrer-Policy",
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    // Basic CSP
    headers.insert(
        "Content-Security-Policy",
        HeaderValue::from_static("default-src 'self'"),
    );

    response
}
