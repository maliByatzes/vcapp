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

    let config = init_configuration().expect("Failed to load configuration.");
    let connection_str = config.database.connection_string();

    // Set up connection pool
    let conn_pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&connection_str)
        .await
        .expect("Failed to connect to database.");

    // Bind address with tokio
    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.application_port)).await?;

    // Call the run function from init
    run(listener, conn_pool).await;

    Ok(())
}
