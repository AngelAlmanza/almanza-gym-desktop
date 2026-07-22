use sqlx::SqlitePool;

use super::models::{CreateUserRequest, UpdateUserRequest};
use super::repository;
use crate::auth::models::UserResponse;
use crate::auth::repository as auth_repo;
use crate::auth::service::hash_password;
use crate::shared::error::AppError;

const VALID_ROLES: &[&str] = &["admin", "manager", "cashier"];

pub async fn list_users(pool: &SqlitePool) -> Result<Vec<UserResponse>, AppError> {
    let users = repository::list_users(pool).await?;
    Ok(users.into_iter().map(UserResponse::from).collect())
}

pub async fn create_user(
    pool: &SqlitePool,
    request: CreateUserRequest,
    actor_role: &str,
) -> Result<UserResponse, AppError> {
    // Validate role
    if !VALID_ROLES.contains(&request.role.as_str()) {
        return Err(AppError::Validation(format!(
            "Rol invalido: {}",
            request.role
        )));
    }

    // Manager can only create cashier
    if actor_role == "manager" && request.role != "cashier" {
        return Err(AppError::PermissionDenied);
    }

    // Only admin can create users (except manager creating cashier)
    if actor_role != "admin" && actor_role != "manager" {
        return Err(AppError::PermissionDenied);
    }

    if request.username.trim().is_empty() {
        return Err(AppError::Validation(
            "El nombre de usuario es requerido".into(),
        ));
    }

    if request.full_name.trim().is_empty() {
        return Err(AppError::Validation(
            "El nombre completo es requerido".into(),
        ));
    }

    if request.password.len() < 8 {
        return Err(AppError::Validation(
            "La contrasena debe tener al menos 8 caracteres".into(),
        ));
    }

    if repository::username_exists(pool, request.username.trim()).await? {
        return Err(AppError::Validation(
            "El nombre de usuario ya existe".into(),
        ));
    }

    let password_hash = hash_password(&request.password)?;
    let user = repository::create_user(
        pool,
        request.username.trim(),
        request.full_name.trim(),
        &password_hash,
        &request.role,
    )
    .await?;

    Ok(UserResponse::from(user))
}

pub async fn update_user(
    pool: &SqlitePool,
    user_id: i64,
    request: UpdateUserRequest,
    actor_id: i64,
    actor_role: &str,
) -> Result<UserResponse, AppError> {
    if actor_role != "admin" {
        return Err(AppError::PermissionDenied);
    }

    let target = repository::find_user_by_id(pool, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Usuario no encontrado".into()))?;

    // Cannot change own role
    if actor_id == target.id {
        if let Some(ref new_role) = request.role {
            if *new_role != target.role {
                return Err(AppError::Validation(
                    "No puedes cambiar tu propio rol".into(),
                ));
            }
        }
    }

    if let Some(ref role) = request.role {
        if !VALID_ROLES.contains(&role.as_str()) {
            return Err(AppError::Validation(format!("Rol invalido: {}", role)));
        }
    }

    let password_hash = match request.password {
        Some(ref pwd) => {
            if pwd.len() < 8 {
                return Err(AppError::Validation(
                    "La contrasena debe tener al menos 8 caracteres".into(),
                ));
            }
            Some(hash_password(pwd)?)
        }
        None => None,
    };

    let user = repository::update_user(
        pool,
        user_id,
        request.full_name.as_deref(),
        password_hash.as_deref(),
        request.role.as_deref(),
    )
    .await?;

    Ok(UserResponse::from(user))
}

pub async fn deactivate_user(
    pool: &SqlitePool,
    user_id: i64,
    actor_id: i64,
    actor_role: &str,
) -> Result<(), AppError> {
    if actor_role != "admin" {
        return Err(AppError::PermissionDenied);
    }

    if actor_id == user_id {
        return Err(AppError::Validation(
            "No puedes desactivar tu propia cuenta".into(),
        ));
    }

    let _ = repository::find_user_by_id(pool, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Usuario no encontrado".into()))?;

    repository::deactivate_user(pool, user_id).await?;

    // Invalidate all sessions for the deactivated user
    auth_repo::delete_sessions_by_user_id(pool, user_id).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::models::SetupRequest;
    use crate::auth::service::setup_admin;
    use crate::shared::database::init_test_db;

    async fn setup_test_admin(pool: &SqlitePool) -> (UserResponse, String) {
        setup_admin(
            pool,
            SetupRequest {
                username: "admin".into(),
                full_name: "Admin".into(),
                password: "password123".into(),
            },
        )
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn test_admin_can_create_any_role() {
        let pool = init_test_db().await.unwrap();
        let (admin, _) = setup_test_admin(&pool).await;

        for role in &["admin", "manager", "cashier"] {
            let req = CreateUserRequest {
                username: format!("user_{}", role),
                full_name: format!("User {}", role),
                password: "password123".into(),
                role: role.to_string(),
            };
            let result = create_user(&pool, req, &admin.role).await;
            assert!(result.is_ok(), "Admin should create {} role", role);
        }
    }

    #[tokio::test]
    async fn test_manager_can_only_create_cashier() {
        let pool = init_test_db().await.unwrap();
        setup_test_admin(&pool).await;

        // Create a manager
        let manager_req = CreateUserRequest {
            username: "manager1".into(),
            full_name: "Manager".into(),
            password: "password123".into(),
            role: "manager".into(),
        };
        create_user(&pool, manager_req, "admin").await.unwrap();

        // Manager creates cashier - OK
        let cashier_req = CreateUserRequest {
            username: "cashier1".into(),
            full_name: "Cashier".into(),
            password: "password123".into(),
            role: "cashier".into(),
        };
        assert!(create_user(&pool, cashier_req, "manager").await.is_ok());

        // Manager tries to create admin - FAIL
        let admin_req = CreateUserRequest {
            username: "admin2".into(),
            full_name: "Admin 2".into(),
            password: "password123".into(),
            role: "admin".into(),
        };
        assert!(create_user(&pool, admin_req, "manager").await.is_err());
    }

    #[tokio::test]
    async fn test_cannot_deactivate_self() {
        let pool = init_test_db().await.unwrap();
        let (admin, _) = setup_test_admin(&pool).await;

        let result = deactivate_user(&pool, admin.id, admin.id, "admin").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cannot_change_own_role() {
        let pool = init_test_db().await.unwrap();
        let (admin, _) = setup_test_admin(&pool).await;

        let req = UpdateUserRequest {
            full_name: None,
            password: None,
            role: Some("cashier".into()),
        };
        let result = update_user(&pool, admin.id, req, admin.id, "admin").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_duplicate_username_rejected() {
        let pool = init_test_db().await.unwrap();
        setup_test_admin(&pool).await;

        let req = CreateUserRequest {
            username: "admin".into(),
            full_name: "Another Admin".into(),
            password: "password123".into(),
            role: "cashier".into(),
        };
        let result = create_user(&pool, req, "admin").await;
        assert!(result.is_err());
    }
}
