use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;
use vcapp::init::init_app;

#[tokio::test]
async fn health_check_works() {
    let app = init_app();

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

#[tokio::test]
async fn handler_404_works() {
    let app = init_app();
    let test_cases = vec![
        "invalid_route",
        "very_very_very_invalid_route",
        "gfhbhbvhehfdncjdv",
    ];

    for route in test_cases {
        let response = app
            .clone() // NOTE: Avoid the use of .clone() here
            .oneshot(
                Request::builder()
                    .uri(format!("/{}", route))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Invalid route, nothing here.");
    }
}
