use validator::validate_email;

use crate::errors::RegisterError;

// Validate user's email
#[derive(Debug)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn parse(s: String) -> Result<String, RegisterError> {
        // Using validate_email should suffice for now
        if validate_email(&s) {
            Ok(s)
        } else {
            Err(RegisterError::EmailInvalid)
        }
    }
}

impl AsRef<str> for UserEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// Validate user's password
// WARNING: ABOLISH THE REGEX AND FIND ANOTHER WAY TO VALIDATE THE PASSWORD
// static PASSWORD_REGEX: Lazy<Regex> = Lazy::new(|| {
// Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]+$").unwrap()
// });

#[derive(Debug, PartialEq)]
pub struct UserPassword(String);

impl UserPassword {
    pub fn parse(s: String) -> Result<String, RegisterError> {
        // Requirements:
        // Not empty
        // Min length of 8 chars
        // Max length of 256 chars
        // Complexity: contains at least a lowercase, uppercase, number and special character

        let is_empty_or_ws = s.trim().is_empty();
        let is_too_short = s.chars().count() < 8;
        let is_too_long = s.chars().count() > 256;

        if is_empty_or_ws {
            Err(RegisterError::PasswordEmpty)
        } else if is_too_short {
            Err(RegisterError::PasswordTooShort)
        } else if is_too_long {
            Err(RegisterError::PassowrdTooLong)
        } else {
            Ok(s)
        }
    }
}

// Validate user's username
#[derive(Debug)]
pub struct UserUsername(String);

impl UserUsername {
    pub fn parse(s: String) -> Result<String, RegisterError> {
        // Requirements
        // Not empty
        // Min length of 6 chars
        // Max length of 256 chars

        let is_empty_or_ws = s.trim().is_empty();
        let is_too_short = s.chars().count() < 6;
        let is_too_long = s.chars().count() > 256;

        if is_empty_or_ws {
            Err(RegisterError::UsernameEmpty)
        } else if is_too_short {
            Err(RegisterError::UsernameTooShort)
        } else if is_too_long {
            Err(RegisterError::UsernameTooLong)
        } else {
            Ok(s)
        }
    }
}

// TODO: Implements tests for different combinations of strings
