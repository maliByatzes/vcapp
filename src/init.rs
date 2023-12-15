use crate::routes::{handler_404, health_check};
use axum::{routing::get, Router};
use tokio::net::TcpListener;

pub async fn run(listener: TcpListener) {
    axum::serve(listener, init_app()).await.unwrap();
}

pub fn init_app() -> Router {
    // Initialize application
    let app = Router::new().route("/health_check", get(health_check));

    // Add fallback for invalid routes
    app.fallback(handler_404)
}
