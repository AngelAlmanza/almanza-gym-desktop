use sqlx::SqlitePool;

use super::models::{Session, User};

pub async fn count_users(pool: &SqlitePool) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

pub async fn find_user_by_username(
    pool: &SqlitePool,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(pool)
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

pub async fn create_session(
    pool: &SqlitePool,
    user_id: i64,
    token: &str,
) -> Result<Session, sqlx::Error> {
    sqlx::query_as::<_, Session>(
        "INSERT INTO sessions (user_id, token) VALUES (?, ?) RETURNING *",
    )
    .bind(user_id)
    .bind(token)
    .fetch_one(pool)
    .await
}

pub async fn find_session_by_token(
    pool: &SqlitePool,
    token: &str,
) -> Result<Option<(Session, User)>, sqlx::Error> {
    let row = sqlx::query_as::<_, Session>("SELECT * FROM sessions WHERE token = ?")
        .bind(token)
        .fetch_optional(pool)
        .await?;

    match row {
        Some(session) => {
            let user = find_user_by_id(pool, session.user_id).await?;
            match user {
                Some(u) => Ok(Some((session, u))),
                None => Ok(None),
            }
        }
        None => Ok(None),
    }
}

pub async fn delete_session_by_token(
    pool: &SqlitePool,
    token: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM sessions WHERE token = ?")
        .bind(token)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_sessions_by_user_id(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM sessions WHERE user_id = ?")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}
