// Create a new user - Steps
// Recieve form data in "html query" format, e.g: `name=..&password=...&username=...`
// Parse the form data to relevant fields of NewUser struct
// Data validation**
// Check for duplicates for unique fields
// Hash the password before storing
// Insert Data to the database
// Process return status**

use axum::{http::StatusCode, Extension, Json};
use sqlx::PgPool;

use crate::{
    errors::RegisterError,
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
pub async fn register_user(
    Json(req): Json<RegisterData>,
    db_pool: Extension<PgPool>,
) -> Result<StatusCode, RegisterError> {
    let new_user = parse_new_user(req)?;

    // Hash password
    let hashed_password = match crate::utils::hash(new_user.password).await {
        Ok(hs) => hs,
        Err(e) => return Err(RegisterError::HashError(e)),
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
    .execute(&*db_pool)
    .await
    {
        Ok(_) => (),
        Err(e) => return Err(RegisterError::DatabaseError(e)),
    };

    Ok(StatusCode::OK)
}
