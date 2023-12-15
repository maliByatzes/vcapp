use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use vcapp::init::run;

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

    // Bind address with tokio
    let listener = TcpListener::bind("0.0.0.0:7777").await?;

    // Call the run function from init
    run(listener).await;

    Ok(())
}
