use tauri::State;

use super::models::{LoginRequest, SetupRequest, UserResponse};
use super::service;
use crate::shared::error::AppError;
use crate::shared::state::AppState;

#[tauri::command]
pub async fn has_users(state: State<'_, AppState>) -> Result<bool, AppError> {
    service::has_users(&state.db).await
}

#[tauri::command]
pub async fn setup_admin(
    state: State<'_, AppState>,
    request: SetupRequest,
) -> Result<(UserResponse, String), AppError> {
    service::setup_admin(&state.db, request).await
}

#[tauri::command]
pub async fn login(
    state: State<'_, AppState>,
    request: LoginRequest,
) -> Result<(UserResponse, String), AppError> {
    service::login(&state.db, request).await
}

#[tauri::command]
pub async fn logout(
    state: State<'_, AppState>,
    token: String,
) -> Result<(), AppError> {
    service::logout(&state.db, token).await
}

#[tauri::command]
pub async fn validate_session(
    state: State<'_, AppState>,
    token: String,
) -> Result<UserResponse, AppError> {
    service::validate_session(&state.db, token).await
}
