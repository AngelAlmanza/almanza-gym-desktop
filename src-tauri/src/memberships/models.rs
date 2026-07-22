use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct MembershipType {
    pub id: i64,
    #[sqlx(rename = "type")]
    #[serde(rename = "type")]
    pub membership_type: String,
    pub name: String,
    pub price: f64,
    pub duration_days: i32,
    pub is_active: bool,
    pub updated_at: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct Membership {
    pub id: i64,
    pub member_id: i64,
    pub membership_type_id: i64,
    pub status: String,
    pub price_paid: f64,
    pub original_price: f64,
    pub start_date: String,
    pub end_date: String,
    pub student_credential_verified: bool,
    pub promotion_id: Option<i64>,
    pub created_at: String,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct MembershipResponse {
    pub id: i64,
    pub member_id: i64,
    pub membership_type_name: String,
    pub membership_type: String,
    pub status: String,
    pub price_paid: f64,
    pub original_price: f64,
    pub start_date: String,
    pub end_date: String,
    pub student_credential_verified: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct ExpiringMembershipInfo {
    pub membership_id: i64,
    pub member_id: i64,
    pub member_name: String,
    pub membership_type_name: String,
    pub end_date: String,
    pub days_remaining: i64,
}

#[derive(Debug, Deserialize)]
pub struct AssignMembershipRequest {
    pub member_id: i64,
    pub membership_type_id: i64,
    pub student_credential_verified: bool,
}

#[derive(Debug, Deserialize)]
pub struct RenewMembershipRequest {
    pub member_id: i64,
    pub membership_type_id: i64,
    pub student_credential_verified: bool,
}
