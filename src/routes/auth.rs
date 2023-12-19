// Create a new user - Steps
// Recieve form data in "html query" format, e.g: `name=..&password=...&username=...`
// Parse the form data to relevant fields of NewUser struct
// Data validation**
// Check for duplicates for unique fields
// Hash the password before storing
// Insert Data to the database
// Process return status**

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
