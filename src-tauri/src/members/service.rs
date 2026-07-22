use rand::Rng;
use sqlx::SqlitePool;

use super::models::{CreateMemberRequest, MemberResponse, UpdateMemberRequest};
use super::repository;
use crate::shared::error::AppError;

pub async fn generate_access_code(pool: &SqlitePool) -> Result<String, AppError> {
    for _ in 0..10 {
        let code = {
            let mut rng = rand::thread_rng();
            format!("{:06}", rng.gen_range(0..1_000_000))
        };
        if !repository::access_code_exists(pool, &code).await? {
            return Ok(code);
        }
    }
    Err(AppError::Validation(
        "No se pudo generar un codigo de acceso unico. Intente de nuevo.".into(),
    ))
}

pub async fn list_members(pool: &SqlitePool) -> Result<Vec<MemberResponse>, AppError> {
    let members = repository::list_members(pool).await?;
    Ok(members.into_iter().map(MemberResponse::from).collect())
}

pub async fn get_member(pool: &SqlitePool, id: i64) -> Result<MemberResponse, AppError> {
    let member = repository::find_member_by_id(pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Miembro no encontrado".into()))?;
    Ok(MemberResponse::from(member))
}

pub async fn search_members(
    pool: &SqlitePool,
    query: String,
) -> Result<Vec<MemberResponse>, AppError> {
    if query.trim().is_empty() {
        return list_members(pool).await;
    }
    let members = repository::search_members(pool, query.trim()).await?;
    Ok(members.into_iter().map(MemberResponse::from).collect())
}

pub async fn create_member(
    pool: &SqlitePool,
    request: CreateMemberRequest,
) -> Result<MemberResponse, AppError> {
    if request.full_name.trim().is_empty() {
        return Err(AppError::Validation(
            "El nombre completo es requerido".into(),
        ));
    }
    if request.phone.trim().is_empty() {
        return Err(AppError::Validation("El telefono es requerido".into()));
    }
    if request.date_of_birth.trim().is_empty() {
        return Err(AppError::Validation(
            "La fecha de nacimiento es requerida".into(),
        ));
    }

    let access_code = generate_access_code(pool).await?;

    let member = repository::create_member(
        pool,
        request.full_name.trim(),
        request.phone.trim(),
        request.email.as_deref(),
        request.date_of_birth.trim(),
        request.emergency_contact.as_deref(),
        request.photo_path.as_deref(),
        &access_code,
    )
    .await?;

    Ok(MemberResponse::from(member))
}

pub async fn update_member(
    pool: &SqlitePool,
    id: i64,
    request: UpdateMemberRequest,
) -> Result<MemberResponse, AppError> {
    let _ = repository::find_member_by_id(pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Miembro no encontrado".into()))?;

    let member = repository::update_member(
        pool,
        id,
        request.full_name.as_deref(),
        request.phone.as_deref(),
        request.email.as_deref(),
        request.date_of_birth.as_deref(),
        request.emergency_contact.as_deref(),
        request.photo_path.as_deref(),
    )
    .await?;

    Ok(MemberResponse::from(member))
}

pub async fn deactivate_member(pool: &SqlitePool, id: i64) -> Result<(), AppError> {
    let _ = repository::find_member_by_id(pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Miembro no encontrado".into()))?;

    repository::deactivate_member(pool, id).await?;
    Ok(())
}

pub async fn regenerate_access_code(
    pool: &SqlitePool,
    id: i64,
) -> Result<String, AppError> {
    let _ = repository::find_member_by_id(pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Miembro no encontrado".into()))?;

    let new_code = generate_access_code(pool).await?;
    repository::update_access_code(pool, id, &new_code).await?;
    Ok(new_code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::database::init_test_db;

    #[tokio::test]
    async fn test_generate_access_code_is_6_digits() {
        let pool = init_test_db().await.unwrap();
        let code = generate_access_code(&pool).await.unwrap();
        assert_eq!(code.len(), 6);
        assert!(code.chars().all(|c| c.is_ascii_digit()));
    }

    #[tokio::test]
    async fn test_create_member_generates_unique_code() {
        let pool = init_test_db().await.unwrap();
        let req1 = CreateMemberRequest {
            full_name: "Juan Perez".into(),
            phone: "5551234567".into(),
            email: None,
            date_of_birth: "1990-01-15".into(),
            emergency_contact: None,
            photo_path: None,
        };
        let req2 = CreateMemberRequest {
            full_name: "Maria Lopez".into(),
            phone: "5559876543".into(),
            email: None,
            date_of_birth: "1985-06-20".into(),
            emergency_contact: None,
            photo_path: None,
        };

        let m1 = create_member(&pool, req1).await.unwrap();
        let m2 = create_member(&pool, req2).await.unwrap();
        assert_ne!(m1.access_code, m2.access_code);
    }

    #[tokio::test]
    async fn test_search_by_name() {
        let pool = init_test_db().await.unwrap();
        let req = CreateMemberRequest {
            full_name: "Carlos Almanza".into(),
            phone: "5551234567".into(),
            email: None,
            date_of_birth: "1990-01-15".into(),
            emergency_contact: None,
            photo_path: None,
        };
        create_member(&pool, req).await.unwrap();

        let results = search_members(&pool, "Carlos".into()).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].full_name, "Carlos Almanza");
    }

    #[tokio::test]
    async fn test_search_by_phone() {
        let pool = init_test_db().await.unwrap();
        let req = CreateMemberRequest {
            full_name: "Ana Garcia".into(),
            phone: "5559999888".into(),
            email: None,
            date_of_birth: "1995-03-10".into(),
            emergency_contact: None,
            photo_path: None,
        };
        create_member(&pool, req).await.unwrap();

        let results = search_members(&pool, "9999888".into()).await.unwrap();
        assert_eq!(results.len(), 1);
    }

    #[tokio::test]
    async fn test_search_by_access_code() {
        let pool = init_test_db().await.unwrap();
        let req = CreateMemberRequest {
            full_name: "Pedro Martinez".into(),
            phone: "5551111222".into(),
            email: None,
            date_of_birth: "1988-12-01".into(),
            emergency_contact: None,
            photo_path: None,
        };
        let member = create_member(&pool, req).await.unwrap();

        let results = search_members(&pool, member.access_code.clone()).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, member.id);
    }

    #[tokio::test]
    async fn test_regenerate_access_code_changes_code() {
        let pool = init_test_db().await.unwrap();
        let req = CreateMemberRequest {
            full_name: "Luis Hernandez".into(),
            phone: "5553334444".into(),
            email: None,
            date_of_birth: "1992-07-25".into(),
            emergency_contact: None,
            photo_path: None,
        };
        let member = create_member(&pool, req).await.unwrap();
        let old_code = member.access_code;

        let new_code = regenerate_access_code(&pool, member.id).await.unwrap();
        assert_ne!(old_code, new_code);
        assert_eq!(new_code.len(), 6);
    }

    #[tokio::test]
    async fn test_deactivate_member() {
        let pool = init_test_db().await.unwrap();
        let req = CreateMemberRequest {
            full_name: "Rosa Jimenez".into(),
            phone: "5555556666".into(),
            email: None,
            date_of_birth: "1993-11-30".into(),
            emergency_contact: None,
            photo_path: None,
        };
        let member = create_member(&pool, req).await.unwrap();

        deactivate_member(&pool, member.id).await.unwrap();

        // Should not appear in active list
        let active = list_members(&pool).await.unwrap();
        assert!(active.iter().all(|m| m.id != member.id));
    }
}
