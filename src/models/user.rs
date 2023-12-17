use serde::Serialize;
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, sqlx::FromRow, Serialize, Clone)]
pub struct User {
    id: i64,
    email: String,
    password: String,
    username: String,
    password_changed_at: NaiveDateTime,
    created_at: NaiveDateTime,
}
