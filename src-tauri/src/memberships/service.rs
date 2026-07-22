use chrono::NaiveDate;
use sqlx::SqlitePool;

use super::models::{
    AssignMembershipRequest, ExpiringMembershipInfo, MembershipResponse, MembershipType,
    RenewMembershipRequest,
};
use super::repository;
use crate::members::repository as members_repo;
use crate::shared::error::AppError;

pub async fn list_membership_types(
    pool: &SqlitePool,
) -> Result<Vec<MembershipType>, AppError> {
    // Expire overdue first
    repository::expire_overdue_memberships(pool).await?;
    let types = repository::list_membership_types(pool).await?;
    Ok(types)
}

pub async fn update_membership_type_price(
    pool: &SqlitePool,
    type_id: i64,
    price: f64,
) -> Result<MembershipType, AppError> {
    if price < 0.0 {
        return Err(AppError::Validation("El precio no puede ser negativo".into()));
    }

    let _ = repository::find_membership_type_by_id(pool, type_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Tipo de membresia no encontrado".into()))?;

    let updated = repository::update_membership_type_price(pool, type_id, price).await?;
    Ok(updated)
}

pub async fn assign_membership(
    pool: &SqlitePool,
    request: AssignMembershipRequest,
) -> Result<MembershipResponse, AppError> {
    // Expire overdue first
    repository::expire_overdue_memberships(pool).await?;

    // Verify member exists and is active
    let member = members_repo::find_member_by_id(pool, request.member_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Miembro no encontrado".into()))?;

    if !member.is_active {
        return Err(AppError::Validation("El miembro esta desactivado".into()));
    }

    // Verify membership type
    let mtype = repository::find_membership_type_by_id(pool, request.membership_type_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Tipo de membresia no encontrado".into()))?;

    if !mtype.is_active {
        return Err(AppError::Validation(
            "El tipo de membresia esta desactivado".into(),
        ));
    }

    if mtype.price <= 0.0 {
        return Err(AppError::Validation(
            "Configure el precio de la membresia antes de asignarla".into(),
        ));
    }

    // Student verification
    if mtype.membership_type == "student" && !request.student_credential_verified {
        return Err(AppError::Validation(
            "Se requiere verificacion de credencial de estudiante".into(),
        ));
    }

    // Check if there's an active membership - expire it
    if let Some(active) = repository::find_active_membership(pool, request.member_id).await? {
        repository::expire_membership(pool, active.id).await?;
    }

    // Calculate dates
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let start = NaiveDate::parse_from_str(&today, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Error al calcular fecha".into()))?;
    let end = start + chrono::Duration::days(mtype.duration_days as i64);
    let end_date = end.format("%Y-%m-%d").to_string();

    repository::create_membership(
        pool,
        request.member_id,
        request.membership_type_id,
        "active",
        mtype.price,
        mtype.price,
        &today,
        &end_date,
        request.student_credential_verified,
    )
    .await?;

    // Return the latest membership with type info
    let memberships = repository::list_memberships_by_member(pool, request.member_id).await?;
    memberships
        .into_iter()
        .next()
        .ok_or_else(|| AppError::Validation("Error al obtener la membresia creada".into()))
}

pub async fn renew_membership(
    pool: &SqlitePool,
    request: RenewMembershipRequest,
) -> Result<MembershipResponse, AppError> {
    // Expire overdue first
    repository::expire_overdue_memberships(pool).await?;

    // Verify member
    let member = members_repo::find_member_by_id(pool, request.member_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Miembro no encontrado".into()))?;

    if !member.is_active {
        return Err(AppError::Validation("El miembro esta desactivado".into()));
    }

    // Verify membership type
    let mtype = repository::find_membership_type_by_id(pool, request.membership_type_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Tipo de membresia no encontrado".into()))?;

    if mtype.price <= 0.0 {
        return Err(AppError::Validation(
            "Configure el precio de la membresia antes de renovar".into(),
        ));
    }

    if mtype.membership_type == "student" && !request.student_credential_verified {
        return Err(AppError::Validation(
            "Se requiere verificacion de credencial de estudiante".into(),
        ));
    }

    // Find current/latest membership
    let latest = repository::find_latest_membership(pool, request.member_id).await?;

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let today_date = NaiveDate::parse_from_str(&today, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Error al calcular fecha".into()))?;

    let (start_date, end_date) = if let Some(ref current) = latest {
        if current.status == "active" {
            // Early renewal: extends from current end_date
            let current_end = NaiveDate::parse_from_str(&current.end_date, "%Y-%m-%d")
                .map_err(|_| AppError::Validation("Error al leer fecha de vencimiento".into()))?;
            let new_end = current_end + chrono::Duration::days(mtype.duration_days as i64);
            (
                current.end_date.clone(),
                new_end.format("%Y-%m-%d").to_string(),
            )
        } else {
            // Expired/cancelled: starts from today
            let new_end = today_date + chrono::Duration::days(mtype.duration_days as i64);
            (today.clone(), new_end.format("%Y-%m-%d").to_string())
        }
    } else {
        // No previous membership - treat as new assignment
        let new_end = today_date + chrono::Duration::days(mtype.duration_days as i64);
        (today.clone(), new_end.format("%Y-%m-%d").to_string())
    };

    // Expire the current active membership if exists
    if let Some(ref current) = latest {
        if current.status == "active" {
            repository::expire_membership(pool, current.id).await?;
        }
    }

    repository::create_membership(
        pool,
        request.member_id,
        request.membership_type_id,
        "active",
        mtype.price,
        mtype.price,
        &start_date,
        &end_date,
        request.student_credential_verified,
    )
    .await?;

    let memberships = repository::list_memberships_by_member(pool, request.member_id).await?;
    memberships
        .into_iter()
        .next()
        .ok_or_else(|| AppError::Validation("Error al obtener la membresia renovada".into()))
}

pub async fn get_member_memberships(
    pool: &SqlitePool,
    member_id: i64,
) -> Result<Vec<MembershipResponse>, AppError> {
    repository::expire_overdue_memberships(pool).await?;
    let memberships = repository::list_memberships_by_member(pool, member_id).await?;
    Ok(memberships)
}

pub async fn get_expiring_memberships(
    pool: &SqlitePool,
) -> Result<Vec<ExpiringMembershipInfo>, AppError> {
    repository::expire_overdue_memberships(pool).await?;
    let expiring = repository::find_expiring_memberships(pool, 3).await?;
    Ok(expiring)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::members::models::CreateMemberRequest;
    use crate::members::service::create_member;
    use crate::shared::database::init_test_db;

    async fn setup_member(pool: &SqlitePool) -> i64 {
        let member = create_member(
            pool,
            CreateMemberRequest {
                full_name: "Test Member".into(),
                phone: "5551234567".into(),
                email: None,
                date_of_birth: "1990-01-01".into(),
                emergency_contact: None,
                photo_path: None,
            },
        )
        .await
        .unwrap();
        member.id
    }

    async fn configure_prices(pool: &SqlitePool) {
        update_membership_type_price(pool, 1, 500.0).await.unwrap();
        update_membership_type_price(pool, 2, 350.0).await.unwrap();
    }

    #[tokio::test]
    async fn test_assign_membership_calculates_correct_end_date() {
        let pool = init_test_db().await.unwrap();
        let member_id = setup_member(&pool).await;
        configure_prices(&pool).await;

        let result = assign_membership(
            &pool,
            AssignMembershipRequest {
                member_id,
                membership_type_id: 1, // Normal, 30 days
                student_credential_verified: false,
            },
        )
        .await
        .unwrap();

        assert_eq!(result.status, "active");

        let start = NaiveDate::parse_from_str(&result.start_date, "%Y-%m-%d").unwrap();
        let end = NaiveDate::parse_from_str(&result.end_date, "%Y-%m-%d").unwrap();
        assert_eq!((end - start).num_days(), 30);
    }

    #[tokio::test]
    async fn test_assign_membership_fails_if_price_not_configured() {
        let pool = init_test_db().await.unwrap();
        let member_id = setup_member(&pool).await;
        // Don't configure prices (they're 0 by default)

        let result = assign_membership(
            &pool,
            AssignMembershipRequest {
                member_id,
                membership_type_id: 1,
                student_credential_verified: false,
            },
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_student_membership_requires_verification() {
        let pool = init_test_db().await.unwrap();
        let member_id = setup_member(&pool).await;
        configure_prices(&pool).await;

        let result = assign_membership(
            &pool,
            AssignMembershipRequest {
                member_id,
                membership_type_id: 2, // Student
                student_credential_verified: false,
            },
        )
        .await;

        assert!(result.is_err());

        // With verification
        let result = assign_membership(
            &pool,
            AssignMembershipRequest {
                member_id,
                membership_type_id: 2,
                student_credential_verified: true,
            },
        )
        .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_early_renewal_extends_from_current_end_date() {
        let pool = init_test_db().await.unwrap();
        let member_id = setup_member(&pool).await;
        configure_prices(&pool).await;

        // Assign first membership
        let first = assign_membership(
            &pool,
            AssignMembershipRequest {
                member_id,
                membership_type_id: 1,
                student_credential_verified: false,
            },
        )
        .await
        .unwrap();

        // Renew while active (early renewal)
        let renewed = renew_membership(
            &pool,
            RenewMembershipRequest {
                member_id,
                membership_type_id: 1,
                student_credential_verified: false,
            },
        )
        .await
        .unwrap();

        // Start date should be the previous end date
        assert_eq!(renewed.start_date, first.end_date);

        let renewed_start =
            NaiveDate::parse_from_str(&renewed.start_date, "%Y-%m-%d").unwrap();
        let renewed_end = NaiveDate::parse_from_str(&renewed.end_date, "%Y-%m-%d").unwrap();
        assert_eq!((renewed_end - renewed_start).num_days(), 30);
    }

    #[tokio::test]
    async fn test_late_renewal_starts_from_today() {
        let pool = init_test_db().await.unwrap();
        let member_id = setup_member(&pool).await;
        configure_prices(&pool).await;

        // Create a membership and manually expire it
        assign_membership(
            &pool,
            AssignMembershipRequest {
                member_id,
                membership_type_id: 1,
                student_credential_verified: false,
            },
        )
        .await
        .unwrap();

        // Force expire
        sqlx::query("UPDATE memberships SET status = 'expired', end_date = '2020-01-01' WHERE member_id = ?")
            .bind(member_id)
            .execute(&pool)
            .await
            .unwrap();

        // Renew after expired
        let renewed = renew_membership(
            &pool,
            RenewMembershipRequest {
                member_id,
                membership_type_id: 1,
                student_credential_verified: false,
            },
        )
        .await
        .unwrap();

        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        assert_eq!(renewed.start_date, today);
    }

    #[tokio::test]
    async fn test_expire_overdue_memberships() {
        let pool = init_test_db().await.unwrap();
        let member_id = setup_member(&pool).await;
        configure_prices(&pool).await;

        assign_membership(
            &pool,
            AssignMembershipRequest {
                member_id,
                membership_type_id: 1,
                student_credential_verified: false,
            },
        )
        .await
        .unwrap();

        // Set end_date to past
        sqlx::query("UPDATE memberships SET end_date = '2020-01-01' WHERE member_id = ?")
            .bind(member_id)
            .execute(&pool)
            .await
            .unwrap();

        let expired_count = repository::expire_overdue_memberships(&pool).await.unwrap();
        assert_eq!(expired_count, 1);
    }
}
