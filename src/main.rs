use tokio::net::TcpListener;
use vcapp::init::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bind address with tokio
    let listener = TcpListener::bind("0.0.0.0:7777").await?;

    // Call the run function form init
    run(listener).await;

    Ok(())
}
