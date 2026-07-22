use tauri::State;

use super::models::{
    AssignMembershipRequest, ExpiringMembershipInfo, MembershipResponse, MembershipType,
    RenewMembershipRequest,
};
use super::service;
use crate::auth::service::validate_session;
use crate::shared::error::AppError;
use crate::shared::state::AppState;

#[tauri::command]
pub async fn list_membership_types(
    state: State<'_, AppState>,
    token: String,
) -> Result<Vec<MembershipType>, AppError> {
    validate_session(&state.db, token).await?;
    service::list_membership_types(&state.db).await
}

#[tauri::command]
pub async fn update_membership_type_price(
    state: State<'_, AppState>,
    token: String,
    type_id: i64,
    price: f64,
) -> Result<MembershipType, AppError> {
    let actor = validate_session(&state.db, token).await?;
    if actor.role != "admin" {
        return Err(AppError::PermissionDenied);
    }
    service::update_membership_type_price(&state.db, type_id, price).await
}

#[tauri::command]
pub async fn assign_membership(
    state: State<'_, AppState>,
    token: String,
    request: AssignMembershipRequest,
) -> Result<MembershipResponse, AppError> {
    let actor = validate_session(&state.db, token).await?;
    if actor.role != "admin" && actor.role != "manager" {
        return Err(AppError::PermissionDenied);
    }
    service::assign_membership(&state.db, request).await
}

#[tauri::command]
pub async fn renew_membership(
    state: State<'_, AppState>,
    token: String,
    request: RenewMembershipRequest,
) -> Result<MembershipResponse, AppError> {
    let actor = validate_session(&state.db, token).await?;
    if actor.role != "admin" && actor.role != "manager" {
        return Err(AppError::PermissionDenied);
    }
    service::renew_membership(&state.db, request).await
}

#[tauri::command]
pub async fn get_member_memberships(
    state: State<'_, AppState>,
    token: String,
    member_id: i64,
) -> Result<Vec<MembershipResponse>, AppError> {
    validate_session(&state.db, token).await?;
    service::get_member_memberships(&state.db, member_id).await
}

#[tauri::command]
pub async fn get_expiring_memberships(
    state: State<'_, AppState>,
    token: String,
) -> Result<Vec<ExpiringMembershipInfo>, AppError> {
    let actor = validate_session(&state.db, token).await?;
    if actor.role != "admin" && actor.role != "manager" {
        return Err(AppError::PermissionDenied);
    }
    service::get_expiring_memberships(&state.db).await
}
