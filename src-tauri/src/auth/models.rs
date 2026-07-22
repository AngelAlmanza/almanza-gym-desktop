use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub full_name: String,
    pub password_hash: String,
    pub role: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct Session {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
    pub created_at: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub full_name: String,
    pub role: String,
    pub is_active: bool,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            username: u.username,
            full_name: u.full_name,
            role: u.role,
            is_active: u.is_active,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct SetupRequest {
    pub username: String,
    pub full_name: String,
    pub password: String,
}
