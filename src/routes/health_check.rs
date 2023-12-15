use axum::http::StatusCode;

// Health check function that retuns `200 OK`
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
