use crate::routes::{handler_404, health_check};
use axum::{routing::get, Router};
use tokio::net::TcpListener;

pub async fn run(listener: TcpListener) {
    // Initialize application
    let app = Router::new().route("/health_check", get(health_check));

    // Add fallback for invalid routes
    let app = app.fallback(handler_404);

    // Serve the app with axum
    axum::serve(listener, app).await.unwrap();
}
