use axum::{http::StatusCode, response::IntoResponse, Json};

// Register Error enum
#[derive(Debug)]
pub enum RegisterError {
    PasswordMismatch,
    DuplicateEmail,
    DuplicateUsername,
    DatabaseError(String),
}

impl IntoResponse for RegisterError {
    fn into_response(self) -> axum::response::Response {
        match self {
            RegisterError::PasswordMismatch => {
                (StatusCode::BAD_REQUEST, Json("Passwords do not match.")).into_response()
            }
            RegisterError::DuplicateEmail => (
                StatusCode::BAD_REQUEST,
                Json("Email already exist in the database."),
            )
                .into_response(),
            RegisterError::DuplicateUsername => {
                (StatusCode::BAD_REQUEST, Json("Username already exists.")).into_response()
            }
            RegisterError::DatabaseError(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(message)).into_response()
            }
        }
    }
}
