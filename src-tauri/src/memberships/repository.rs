use sqlx::SqlitePool;

use super::models::{ExpiringMembershipInfo, Membership, MembershipResponse, MembershipType};

pub async fn list_membership_types(
    pool: &SqlitePool,
) -> Result<Vec<MembershipType>, sqlx::Error> {
    sqlx::query_as::<_, MembershipType>("SELECT * FROM membership_types ORDER BY id")
        .fetch_all(pool)
        .await
}

pub async fn find_membership_type_by_id(
    pool: &SqlitePool,
    id: i64,
) -> Result<Option<MembershipType>, sqlx::Error> {
    sqlx::query_as::<_, MembershipType>("SELECT * FROM membership_types WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn update_membership_type_price(
    pool: &SqlitePool,
    id: i64,
    price: f64,
) -> Result<MembershipType, sqlx::Error> {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    sqlx::query_as::<_, MembershipType>(
        "UPDATE membership_types SET price = ?, updated_at = ? WHERE id = ? RETURNING *",
    )
    .bind(price)
    .bind(&now)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn create_membership(
    pool: &SqlitePool,
    member_id: i64,
    membership_type_id: i64,
    status: &str,
    price_paid: f64,
    original_price: f64,
    start_date: &str,
    end_date: &str,
    student_credential_verified: bool,
) -> Result<Membership, sqlx::Error> {
    sqlx::query_as::<_, Membership>(
        "INSERT INTO memberships (member_id, membership_type_id, status, price_paid, original_price, start_date, end_date, student_credential_verified) VALUES (?, ?, ?, ?, ?, ?, ?, ?) RETURNING *",
    )
    .bind(member_id)
    .bind(membership_type_id)
    .bind(status)
    .bind(price_paid)
    .bind(original_price)
    .bind(start_date)
    .bind(end_date)
    .bind(student_credential_verified)
    .fetch_one(pool)
    .await
}

pub async fn find_active_membership(
    pool: &SqlitePool,
    member_id: i64,
) -> Result<Option<Membership>, sqlx::Error> {
    sqlx::query_as::<_, Membership>(
        "SELECT * FROM memberships WHERE member_id = ? AND status = 'active' ORDER BY created_at DESC LIMIT 1",
    )
    .bind(member_id)
    .fetch_optional(pool)
    .await
}

pub async fn find_latest_membership(
    pool: &SqlitePool,
    member_id: i64,
) -> Result<Option<Membership>, sqlx::Error> {
    sqlx::query_as::<_, Membership>(
        "SELECT * FROM memberships WHERE member_id = ? ORDER BY id DESC LIMIT 1",
    )
    .bind(member_id)
    .fetch_optional(pool)
    .await
}

pub async fn expire_membership(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE memberships SET status = 'expired' WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn list_memberships_by_member(
    pool: &SqlitePool,
    member_id: i64,
) -> Result<Vec<MembershipResponse>, sqlx::Error> {
    sqlx::query_as::<_, MembershipResponse>(
        "SELECT m.id, m.member_id, mt.name as membership_type_name, mt.type as membership_type, m.status, m.price_paid, m.original_price, m.start_date, m.end_date, m.student_credential_verified, m.created_at FROM memberships m JOIN membership_types mt ON m.membership_type_id = mt.id WHERE m.member_id = ? ORDER BY m.id DESC",
    )
    .bind(member_id)
    .fetch_all(pool)
    .await
}

pub async fn find_expiring_memberships(
    pool: &SqlitePool,
    days: i64,
) -> Result<Vec<ExpiringMembershipInfo>, sqlx::Error> {
    sqlx::query_as::<_, ExpiringMembershipInfo>(
        "SELECT m.id as membership_id, m.member_id, mem.full_name as member_name, mt.name as membership_type_name, m.end_date, CAST(julianday(m.end_date) - julianday('now') AS INTEGER) as days_remaining FROM memberships m JOIN members mem ON m.member_id = mem.id JOIN membership_types mt ON m.membership_type_id = mt.id WHERE m.status = 'active' AND julianday(m.end_date) - julianday('now') <= ? AND julianday(m.end_date) - julianday('now') >= 0 ORDER BY m.end_date ASC",
    )
    .bind(days)
    .fetch_all(pool)
    .await
}

pub async fn expire_overdue_memberships(
    pool: &SqlitePool,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE memberships SET status = 'expired' WHERE status = 'active' AND end_date < date('now')",
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}
