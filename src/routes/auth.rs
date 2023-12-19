// Create a new user - Steps
// Recieve form data in "html query" format, e.g: `name=..&password=...&username=...`
// Parse the form data to relevant fields of NewUser struct
// Data validation**
// Check for duplicates for unique fields
// Hash the password before storing
// Insert Data to the database
// Process return status**

use crate::models::NewUser;

#[derive(Debug, serde::Deserialize)]
pub struct RegisterData {
    email: String,
    password: String,
    password_confirmation: String,
    username: String,
}

// Parse new user to a NewUser struct
// pub fn parse_new_user(form: RegisterData) -> Result<NewUser, String> {}
