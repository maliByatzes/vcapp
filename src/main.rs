use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use vcapp::{config::init_configuration, init::run};

// TODO: Telemety using tracing
// TODO: Error handling with tracing

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize registry for storing Layers
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "vcapp=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize configuration
    let config = init_configuration().expect("Failed to load configuration.");

    // println!("{:?}", config);

    // Set up connection pool
    let conn_pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect_lazy_with(config.database.with_db());

    // Bind address with tokio
    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.application_port)).await?;

    tracing::debug!("Server running on port {}", config.application_port);
    // Call the run function from init
    run(listener, conn_pool).await;

    Ok(())
}
