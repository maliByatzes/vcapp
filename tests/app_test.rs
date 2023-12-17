use axum::body::Body;
use axum::http::Request;
use axum::http::StatusCode;
use axum::Router;
use sqlx::PgPool;
use tower::ServiceExt;
use vcapp::config::init_configuration;
use vcapp::config::DatabaseSettings;
use vcapp::init::init_app;

pub async fn config_database(config: &DatabaseSettings) -> PgPool {
    // // Connect to postgres without database
    // let mut connection = PgConnection::connect_with(&config.without_db())
    //     .await
    //     .expect("Failed to connect to postgres 1");
    //
    // // Create a database with db_name
    // connection
    //     .execute(&*format!(r#"create database "{}";"#, config.db_name))
    //     .await
    //     .expect("Failed to create a database");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to postgres 2");

    // Migrate database
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

async fn spawn_app() -> Router {
    // assign a random port
    // let listener = TcpListener::bind("0.0.0.0:0").expect("Failed to assign a random port");
    // Get the port assigned by the OS
    // let port = listener.local_addr().unwrap().port();
    // let address = format!("http://127.0.0.1:{}", port);

    let config = init_configuration().expect("Failed to load configuration");
    // config.database.db_name = Uuid::new_v4().to_string();
    // println!("{:?}", config);
    let connection_pool = config_database(&config.database).await;

    init_app(connection_pool)
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health_check")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
