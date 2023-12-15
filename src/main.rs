use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // Build the application with one route
    let app = Router::new().route("/", get(root));

    // Run app instance with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handler to return a simple Hello World
async fn root() -> &'static str {
    "Hello, World!"
}
