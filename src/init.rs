use crate::routes::{handler_404, health_check, hello_template};
use axum::{routing::get, Router};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub async fn run(listener: TcpListener, conn_pool: PgPool) {
    axum::serve(listener, init_app(conn_pool)).await.unwrap();
}

pub fn init_app(conn_pool: PgPool) -> Router {
    let assets_path = std::env::current_dir().unwrap();

    // Initialize application
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/", get(hello_template))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .with_state(conn_pool);

    // Add fallback for invalid routes
    app.fallback(handler_404)
}
