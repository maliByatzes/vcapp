use std::sync::Arc;

use crate::routes::{handler_404, health_check, hello_template, register_user};
use axum::{
    routing::{get, post},
    Extension, Router,
};
use sqlx::{PgPool, Pool, Postgres};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<Pool<Postgres>>,
}

pub async fn run(listener: TcpListener, conn_pool: PgPool) {
    axum::serve(listener, init_app(conn_pool)).await.unwrap();
}

pub fn init_app(conn_pool: PgPool) -> Router {
    let assets_path = std::env::current_dir().unwrap();

    let pool = Arc::new(conn_pool);
    let app_state = Arc::new(AppState { pool });

    // Initialize application
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/", get(hello_template))
        .route("/v1/register", post(register_user))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .layer(Extension(app_state));

    // Add fallback for invalid routes
    app.fallback(handler_404)
}
