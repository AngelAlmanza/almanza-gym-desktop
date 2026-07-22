use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use sqlx::SqlitePool;
use uuid::Uuid;

use super::models::{LoginRequest, SetupRequest, UserResponse};
use super::repository;
use crate::shared::error::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Auth(format!("Error al hashear contrasena: {e}")))?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::Auth(format!("Hash invalido: {e}")))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub async fn has_users(pool: &SqlitePool) -> Result<bool, AppError> {
    let count = repository::count_users(pool).await?;
    Ok(count > 0)
}

pub async fn setup_admin(
    pool: &SqlitePool,
    request: SetupRequest,
) -> Result<(UserResponse, String), AppError> {
    if has_users(pool).await? {
        return Err(AppError::Auth(
            "Ya existe al menos un usuario en el sistema".into(),
        ));
    }

    if request.password.len() < 8 {
        return Err(AppError::Validation(
            "La contrasena debe tener al menos 8 caracteres".into(),
        ));
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

    let password_hash = hash_password(&request.password)?;
    let user = repository::create_user(
        pool,
        request.username.trim(),
        request.full_name.trim(),
        &password_hash,
        "admin",
    )
    .await?;

    let token = Uuid::new_v4().to_string();
    repository::create_session(pool, user.id, &token).await?;

    Ok((UserResponse::from(user), token))
}

pub async fn login(
    pool: &SqlitePool,
    request: LoginRequest,
) -> Result<(UserResponse, String), AppError> {
    let generic_error = "Usuario o contrasena incorrectos";

    let user = repository::find_user_by_username(pool, request.username.trim())
        .await?
        .ok_or_else(|| AppError::Auth(generic_error.into()))?;

    if !user.is_active {
        return Err(AppError::Auth(generic_error.into()));
    }

    let valid = verify_password(&request.password, &user.password_hash)?;
    if !valid {
        return Err(AppError::Auth(generic_error.into()));
    }

    let token = Uuid::new_v4().to_string();
    repository::create_session(pool, user.id, &token).await?;

    Ok((UserResponse::from(user), token))
}

pub async fn logout(pool: &SqlitePool, token: String) -> Result<(), AppError> {
    repository::delete_session_by_token(pool, &token).await?;
    Ok(())
}

pub async fn validate_session(
    pool: &SqlitePool,
    token: String,
) -> Result<UserResponse, AppError> {
    let (_, user) = repository::find_session_by_token(pool, &token)
        .await?
        .ok_or_else(|| AppError::Auth("Sesion invalida".into()))?;

    if !user.is_active {
        repository::delete_session_by_token(pool, &token).await?;
        return Err(AppError::Auth("Usuario desactivado".into()));
    }

    Ok(UserResponse::from(user))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::database::init_test_db;

    #[tokio::test]
    async fn test_setup_admin_creates_user_with_admin_role() {
        let pool = init_test_db().await.unwrap();
        let request = SetupRequest {
            username: "admin".into(),
            full_name: "Admin User".into(),
            password: "password123".into(),
        };

        let (user, token) = setup_admin(&pool, request).await.unwrap();
        assert_eq!(user.role, "admin");
        assert_eq!(user.username, "admin");
        assert!(!token.is_empty());
    }

    #[tokio::test]
    async fn test_setup_admin_fails_if_users_exist() {
        let pool = init_test_db().await.unwrap();
        let request = SetupRequest {
            username: "admin".into(),
            full_name: "Admin".into(),
            password: "password123".into(),
        };

        setup_admin(&pool, request).await.unwrap();

        let request2 = SetupRequest {
            username: "admin2".into(),
            full_name: "Admin 2".into(),
            password: "password123".into(),
        };
        let result = setup_admin(&pool, request2).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_setup_admin_rejects_short_password() {
        let pool = init_test_db().await.unwrap();
        let request = SetupRequest {
            username: "admin".into(),
            full_name: "Admin".into(),
            password: "short".into(),
        };

        let result = setup_admin(&pool, request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_login_succeeds_with_valid_credentials() {
        let pool = init_test_db().await.unwrap();
        let setup_req = SetupRequest {
            username: "admin".into(),
            full_name: "Admin".into(),
            password: "password123".into(),
        };
        setup_admin(&pool, setup_req).await.unwrap();

        let login_req = LoginRequest {
            username: "admin".into(),
            password: "password123".into(),
        };
        let (user, token) = login(&pool, login_req).await.unwrap();
        assert_eq!(user.username, "admin");
        assert!(!token.is_empty());
    }

    #[tokio::test]
    async fn test_login_fails_with_wrong_password() {
        let pool = init_test_db().await.unwrap();
        let setup_req = SetupRequest {
            username: "admin".into(),
            full_name: "Admin".into(),
            password: "password123".into(),
        };
        setup_admin(&pool, setup_req).await.unwrap();

        let login_req = LoginRequest {
            username: "admin".into(),
            password: "wrongpassword".into(),
        };
        let result = login(&pool, login_req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_login_fails_with_nonexistent_user() {
        let pool = init_test_db().await.unwrap();
        let login_req = LoginRequest {
            username: "nobody".into(),
            password: "password123".into(),
        };
        let result = login(&pool, login_req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_login_fails_with_inactive_user() {
        let pool = init_test_db().await.unwrap();
        let setup_req = SetupRequest {
            username: "admin".into(),
            full_name: "Admin".into(),
            password: "password123".into(),
        };
        setup_admin(&pool, setup_req).await.unwrap();

        // Deactivate the user
        sqlx::query("UPDATE users SET is_active = 0 WHERE username = 'admin'")
            .execute(&pool)
            .await
            .unwrap();

        let login_req = LoginRequest {
            username: "admin".into(),
            password: "password123".into(),
        };
        let result = login(&pool, login_req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_session_returns_user() {
        let pool = init_test_db().await.unwrap();
        let setup_req = SetupRequest {
            username: "admin".into(),
            full_name: "Admin".into(),
            password: "password123".into(),
        };
        let (_, token) = setup_admin(&pool, setup_req).await.unwrap();

        let user = validate_session(&pool, token).await.unwrap();
        assert_eq!(user.username, "admin");
    }

    #[tokio::test]
    async fn test_logout_invalidates_session() {
        let pool = init_test_db().await.unwrap();
        let setup_req = SetupRequest {
            username: "admin".into(),
            full_name: "Admin".into(),
            password: "password123".into(),
        };
        let (_, token) = setup_admin(&pool, setup_req).await.unwrap();

        logout(&pool, token.clone()).await.unwrap();

        let result = validate_session(&pool, token).await;
        assert!(result.is_err());
    }
}
