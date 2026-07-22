use sqlx::SqlitePool;

use crate::auth::models::User;

pub async fn list_users(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(pool)
        .await
}

pub async fn find_user_by_id(
    pool: &SqlitePool,
    id: i64,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn username_exists(
    pool: &SqlitePool,
    username: &str,
) -> Result<bool, sqlx::Error> {
    let row: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM users WHERE username = ?")
            .bind(username)
            .fetch_one(pool)
            .await?;
    Ok(row.0 > 0)
}

pub async fn create_user(
    pool: &SqlitePool,
    username: &str,
    full_name: &str,
    password_hash: &str,
    role: &str,
) -> Result<User, sqlx::Error> {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    sqlx::query_as::<_, User>(
        "INSERT INTO users (username, full_name, password_hash, role, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, 1, ?, ?) RETURNING *",
    )
    .bind(username)
    .bind(full_name)
    .bind(password_hash)
    .bind(role)
    .bind(&now)
    .bind(&now)
    .fetch_one(pool)
    .await
}

pub async fn update_user(
    pool: &SqlitePool,
    id: i64,
    full_name: Option<&str>,
    password_hash: Option<&str>,
    role: Option<&str>,
) -> Result<User, sqlx::Error> {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

    // Build dynamic update
    let current = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;

    let new_full_name = full_name.unwrap_or(&current.full_name);
    let new_password_hash = password_hash.unwrap_or(&current.password_hash);
    let new_role = role.unwrap_or(&current.role);

    sqlx::query_as::<_, User>(
        "UPDATE users SET full_name = ?, password_hash = ?, role = ?, updated_at = ? WHERE id = ? RETURNING *",
    )
    .bind(new_full_name)
    .bind(new_password_hash)
    .bind(new_role)
    .bind(&now)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn deactivate_user(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    sqlx::query("UPDATE users SET is_active = 0, updated_at = ? WHERE id = ?")
        .bind(&now)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
