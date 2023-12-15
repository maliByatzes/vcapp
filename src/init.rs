use crate::routes::health_check;
use axum::{routing::get, Router};
use tokio::net::TcpListener;

pub async fn run(listener: TcpListener) {
    // Initialize application
    let app = Router::new().route("/health_check", get(health_check));

    // Serve the app with axum
    axum::serve(listener, app).await.unwrap();
}
