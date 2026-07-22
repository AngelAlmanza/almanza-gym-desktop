use sqlx::SqlitePool;

use super::models::Member;

pub async fn list_members(pool: &SqlitePool) -> Result<Vec<Member>, sqlx::Error> {
    sqlx::query_as::<_, Member>(
        "SELECT * FROM members WHERE is_active = 1 ORDER BY full_name ASC",
    )
    .fetch_all(pool)
    .await
}

pub async fn find_member_by_id(
    pool: &SqlitePool,
    id: i64,
) -> Result<Option<Member>, sqlx::Error> {
    sqlx::query_as::<_, Member>("SELECT * FROM members WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn search_members(
    pool: &SqlitePool,
    query: &str,
) -> Result<Vec<Member>, sqlx::Error> {
    let pattern = format!("%{}%", query);
    sqlx::query_as::<_, Member>(
        "SELECT * FROM members WHERE is_active = 1 AND (full_name LIKE ? OR access_code LIKE ? OR phone LIKE ?) ORDER BY full_name ASC",
    )
    .bind(&pattern)
    .bind(&pattern)
    .bind(&pattern)
    .fetch_all(pool)
    .await
}

pub async fn access_code_exists(
    pool: &SqlitePool,
    code: &str,
) -> Result<bool, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM members WHERE access_code = ? AND is_active = 1",
    )
    .bind(code)
    .fetch_one(pool)
    .await?;
    Ok(row.0 > 0)
}

pub async fn create_member(
    pool: &SqlitePool,
    full_name: &str,
    phone: &str,
    email: Option<&str>,
    date_of_birth: &str,
    emergency_contact: Option<&str>,
    photo_path: Option<&str>,
    access_code: &str,
) -> Result<Member, sqlx::Error> {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    sqlx::query_as::<_, Member>(
        "INSERT INTO members (full_name, phone, email, date_of_birth, emergency_contact, photo_path, access_code, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, 1, ?, ?) RETURNING *",
    )
    .bind(full_name)
    .bind(phone)
    .bind(email)
    .bind(date_of_birth)
    .bind(emergency_contact)
    .bind(photo_path)
    .bind(access_code)
    .bind(&now)
    .bind(&now)
    .fetch_one(pool)
    .await
}

pub async fn update_member(
    pool: &SqlitePool,
    id: i64,
    full_name: Option<&str>,
    phone: Option<&str>,
    email: Option<&str>,
    date_of_birth: Option<&str>,
    emergency_contact: Option<&str>,
    photo_path: Option<&str>,
) -> Result<Member, sqlx::Error> {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let current = sqlx::query_as::<_, Member>("SELECT * FROM members WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;

    sqlx::query_as::<_, Member>(
        "UPDATE members SET full_name = ?, phone = ?, email = ?, date_of_birth = ?, emergency_contact = ?, photo_path = ?, updated_at = ? WHERE id = ? RETURNING *",
    )
    .bind(full_name.unwrap_or(&current.full_name))
    .bind(phone.unwrap_or(&current.phone))
    .bind(email.or(current.email.as_deref()))
    .bind(date_of_birth.unwrap_or(&current.date_of_birth))
    .bind(emergency_contact.or(current.emergency_contact.as_deref()))
    .bind(photo_path.or(current.photo_path.as_deref()))
    .bind(&now)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn deactivate_member(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    sqlx::query("UPDATE members SET is_active = 0, updated_at = ? WHERE id = ?")
        .bind(&now)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_access_code(
    pool: &SqlitePool,
    id: i64,
    new_code: &str,
) -> Result<(), sqlx::Error> {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    sqlx::query("UPDATE members SET access_code = ?, updated_at = ? WHERE id = ?")
        .bind(new_code)
        .bind(&now)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
