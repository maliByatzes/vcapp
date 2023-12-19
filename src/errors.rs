use axum::{http::StatusCode, response::IntoResponse, Json};

// Register Error enum
#[derive(Debug)]
pub enum RegisterError {
    PasswordMismatch,
    DuplicateEmail,
    DuplicateUsername,
    EmailInvalid,
    PasswordEmpty,
    PassowrdTooLong,
    PasswordTooShort,
    PasswordNotMatch,
    UsernameEmpty,
    UsernameTooShort,
    UsernameTooLong,
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
            RegisterError::EmailInvalid => {
                (StatusCode::BAD_REQUEST, Json("Email is invalid.")).into_response()
            }
            RegisterError::PasswordEmpty => {
                (StatusCode::BAD_REQUEST, Json("Password is empty.")).into_response()
            }
            RegisterError::PasswordTooShort => (
                StatusCode::BAD_REQUEST,
                Json("Password is too short. Passwords must at least 8 characters."),
            )
                .into_response(),
            RegisterError::PassowrdTooLong => (
                StatusCode::BAD_REQUEST,
                Json("Password is too long. Passwords must be less than 256 characters."),
            )
                .into_response(),
            RegisterError::PasswordNotMatch => {
                (StatusCode::BAD_REQUEST, Json("Invalid password. Pass must contain at least 1 lowercase character, 1 uppercase character, 1 numeric value, 1 special character.")).into_response()
            }
            RegisterError::UsernameEmpty => {
                (StatusCode::BAD_REQUEST, Json("Username is empty.")).into_response()
            }
            RegisterError::UsernameTooShort => (
                StatusCode::BAD_REQUEST,
                Json("Username is too short. Usernames must at least 6 characters long."),
            )
                .into_response(),
            RegisterError::UsernameTooLong => (
                StatusCode::BAD_REQUEST,
                Json("Username is too long. Usernames must be less 256 characters."),
            )
                .into_response(),
            RegisterError::DatabaseError(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(message)).into_response()
            }
        }
    }
}
