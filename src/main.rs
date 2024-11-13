use std::net::SocketAddr;

use axum::{response::IntoResponse, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let router = Router::new().route("/", get(root));

    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind TCP listener");
    axum::serve(listener, router)
        .await
        .expect("Failed to start server");
}

async fn root() -> impl IntoResponse {
    "Hello, World!"
}
