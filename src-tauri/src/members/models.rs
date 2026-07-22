use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Member {
    pub id: i64,
    pub full_name: String,
    pub phone: String,
    pub email: Option<String>,
    pub date_of_birth: String,
    pub emergency_contact: Option<String>,
    pub photo_path: Option<String>,
    pub access_code: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MemberResponse {
    pub id: i64,
    pub full_name: String,
    pub phone: String,
    pub email: Option<String>,
    pub date_of_birth: String,
    pub emergency_contact: Option<String>,
    pub photo_path: Option<String>,
    pub access_code: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Member> for MemberResponse {
    fn from(m: Member) -> Self {
        Self {
            id: m.id,
            full_name: m.full_name,
            phone: m.phone,
            email: m.email,
            date_of_birth: m.date_of_birth,
            emergency_contact: m.emergency_contact,
            photo_path: m.photo_path,
            access_code: m.access_code,
            is_active: m.is_active,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateMemberRequest {
    pub full_name: String,
    pub phone: String,
    pub email: Option<String>,
    pub date_of_birth: String,
    pub emergency_contact: Option<String>,
    pub photo_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemberRequest {
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub date_of_birth: Option<String>,
    pub emergency_contact: Option<String>,
    pub photo_path: Option<String>,
}
