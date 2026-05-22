use axum::{
    http::{header, HeaderValue, Request},
    middleware::Next,
    response::Response,
    Router,
};
use tower_http::cors::CorsLayer;

pub fn add_security_layers<S>(app: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let cors = CorsLayer::new()
        .allow_origin([
            "https://cobaltdev.vercel.app"
                .parse::<HeaderValue>()
                .expect("Invalid CORS origin"),
            "http://localhost:3000"
                .parse::<HeaderValue>()
                .expect("Invalid CORS origin"),
            "http://localhost:8080"
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
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
        .allow_credentials(false);

    app.layer(cors)
        .layer(axum::middleware::from_fn(security_headers_middleware))
}

pub async fn security_headers_middleware(
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();

    headers.insert("X-Frame-Options", HeaderValue::from_static("DENY"));

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
        HeaderValue::from_static("default-src 'self'"),
    );

    response
}
