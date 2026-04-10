mod db;
mod models;
mod handlers;
mod routes;

use dotenvy::dotenv;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = db::init_db().await;
    let app = routes::create_routes(pool);

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}

