mod frontend;
mod models;

#[cfg(not(target_arch = "wasm32"))]
mod server;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    let app = server::create_router();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::info!("🚀 Server running on http://127.0.0.1:3000");
    tracing::info!("📋 Health check: http://127.0.0.1:3000/api/health");
    axum::serve(listener, app).await.unwrap();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(frontend::App);
}