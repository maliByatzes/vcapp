use once_cell::sync::Lazy;
use regex::Regex;
use validator::validate_email;

// Validate user's email
#[derive(Debug)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn parse(s: String) -> Result<UserEmail, String> {
        // Using validate_email should suffice for now
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid email address.", s))
        }
    }
}

impl AsRef<str> for UserEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// Validate user's password

static PASSWORD_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]+$").unwrap()
});

#[derive(Debug)]
pub struct UserPassword(String);

impl UserPassword {
    pub fn parse(s: String) -> Result<UserPassword, String> {
        // Requirements:
        // Not empty
        // Min length of 8 chars
        // Max length of 256 chars
        // Complexity: contains at least a lowercase, uppercase, number and special character

        let is_empty_or_ws = s.trim().is_empty();
        let is_too_short = s.chars().count() < 8;
        let is_too_long = s.chars().count() > 256;
        let is_match = PASSWORD_REGEX.is_match(&s);

        if is_empty_or_ws || is_too_short || is_too_long || is_match {
            Err(format!("{} is not valid password.", s))
        } else {
            Ok(Self(s))
        }
    }
}

// Validate user's username
#[derive(Debug)]
pub struct UserUsername(String);

impl UserUsername {
    pub fn parse(s: String) -> Result<UserUsername, String> {
        // Requirements
        // Not empty
        // Min length of 6 chars
        // Max length of 256 chars

        let is_empty_or_ws = s.trim().is_empty();
        let is_too_short = s.chars().count() < 6;
        let is_too_long = s.chars().count() > 256;

        if is_empty_or_ws || is_too_short || is_too_long {
            Err(format!("{} is not valid username.", s))
        } else {
            Ok(Self(s))
        }
    }
}

// TODO: Implements tests for different combinations of strings
