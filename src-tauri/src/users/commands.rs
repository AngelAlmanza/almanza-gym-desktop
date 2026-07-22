use tauri::State;

use super::models::{CreateUserRequest, UpdateUserRequest};
use super::service;
use crate::auth::models::UserResponse;
use crate::auth::service::validate_session;
use crate::shared::error::AppError;
use crate::shared::state::AppState;

#[tauri::command]
pub async fn list_users(
    state: State<'_, AppState>,
    token: String,
) -> Result<Vec<UserResponse>, AppError> {
    let actor = validate_session(&state.db, token).await?;
    if actor.role != "admin" {
        return Err(AppError::PermissionDenied);
    }
    service::list_users(&state.db).await
}

#[tauri::command]
pub async fn create_user(
    state: State<'_, AppState>,
    token: String,
    request: CreateUserRequest,
) -> Result<UserResponse, AppError> {
    let actor = validate_session(&state.db, token).await?;
    service::create_user(&state.db, request, &actor.role).await
}

#[tauri::command]
pub async fn update_user(
    state: State<'_, AppState>,
    token: String,
    user_id: i64,
    request: UpdateUserRequest,
) -> Result<UserResponse, AppError> {
    let actor = validate_session(&state.db, token).await?;
    service::update_user(&state.db, user_id, request, actor.id, &actor.role).await
}

#[tauri::command]
pub async fn deactivate_user(
    state: State<'_, AppState>,
    token: String,
    user_id: i64,
) -> Result<(), AppError> {
    let actor = validate_session(&state.db, token).await?;
    service::deactivate_user(&state.db, user_id, actor.id, &actor.role).await
}
