use tauri::State;

use super::models::{CreateMemberRequest, MemberResponse, UpdateMemberRequest};
use super::service;
use crate::auth::service::validate_session;
use crate::shared::error::AppError;
use crate::shared::state::AppState;

#[tauri::command]
pub async fn list_members(
    state: State<'_, AppState>,
    token: String,
) -> Result<Vec<MemberResponse>, AppError> {
    validate_session(&state.db, token).await?;
    service::list_members(&state.db).await
}

#[tauri::command]
pub async fn get_member(
    state: State<'_, AppState>,
    token: String,
    member_id: i64,
) -> Result<MemberResponse, AppError> {
    validate_session(&state.db, token).await?;
    service::get_member(&state.db, member_id).await
}

#[tauri::command]
pub async fn search_members(
    state: State<'_, AppState>,
    token: String,
    query: String,
) -> Result<Vec<MemberResponse>, AppError> {
    validate_session(&state.db, token).await?;
    service::search_members(&state.db, query).await
}

#[tauri::command]
pub async fn create_member(
    state: State<'_, AppState>,
    token: String,
    request: CreateMemberRequest,
) -> Result<MemberResponse, AppError> {
    validate_session(&state.db, token).await?;
    service::create_member(&state.db, request).await
}

#[tauri::command]
pub async fn update_member(
    state: State<'_, AppState>,
    token: String,
    member_id: i64,
    request: UpdateMemberRequest,
) -> Result<MemberResponse, AppError> {
    let actor = validate_session(&state.db, token).await?;
    if actor.role != "admin" && actor.role != "manager" {
        return Err(AppError::PermissionDenied);
    }
    service::update_member(&state.db, member_id, request).await
}

#[tauri::command]
pub async fn deactivate_member(
    state: State<'_, AppState>,
    token: String,
    member_id: i64,
) -> Result<(), AppError> {
    let actor = validate_session(&state.db, token).await?;
    if actor.role != "admin" && actor.role != "manager" {
        return Err(AppError::PermissionDenied);
    }
    service::deactivate_member(&state.db, member_id).await
}

#[tauri::command]
pub async fn regenerate_access_code(
    state: State<'_, AppState>,
    token: String,
    member_id: i64,
) -> Result<String, AppError> {
    let actor = validate_session(&state.db, token).await?;
    if actor.role != "admin" && actor.role != "manager" {
        return Err(AppError::PermissionDenied);
    }
    service::regenerate_access_code(&state.db, member_id).await
}
