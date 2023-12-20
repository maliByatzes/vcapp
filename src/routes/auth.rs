use axum::{http::StatusCode, Extension, Json};
use axum_macros::debug_handler;
use std::sync::Arc;

use crate::{
    errors::RegisterError,
    init::AppState,
    models::NewUser,
    validation::{UserEmail, UserPassword, UserUsername},
};

#[derive(Debug, serde::Deserialize)]
pub struct RegisterData {
    email: String,
    password: String,
    password_confirmation: String,
    username: String,
}

// Parse new user to a NewUser struct
pub fn parse_new_user(form: RegisterData) -> Result<NewUser, RegisterError> {
    let email = UserEmail::parse(form.email)?;
    let password = UserPassword::parse(form.password)?;
    let password_confirmation = UserPassword::parse(form.password_confirmation)?;
    let username = UserUsername::parse(form.username)?;

    if password != password_confirmation {
        return Err(RegisterError::PasswordMismatch);
    }

    Ok(NewUser {
        email,
        password,
        username,
    })
}

// Register a new user
// NOTE: Return `Redirect` instead of () later
#[debug_handler]
pub async fn register_user(
    state: Extension<Arc<AppState>>,
    Json(req): Json<RegisterData>,
) -> StatusCode {
    let new_user = parse_new_user(req).unwrap();

    // Hash password
    let hashed_password = match crate::utils::hash(new_user.password).await {
        Ok(hs) => hs,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    match sqlx::query!(
        r#"
            insert into "users"(email, password, username)
            values ($1, $2, $3)
        "#,
        new_user.email,
        hashed_password,
        new_user.username,
    )
    .execute(&*state.pool)
    .await
    {
        Ok(_) => (),
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    StatusCode::OK
}
