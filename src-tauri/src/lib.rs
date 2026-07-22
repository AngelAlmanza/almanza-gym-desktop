mod auth;
mod members;
mod memberships;
mod shared;
mod users;

use shared::database::init_db;
use shared::state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("No se pudo obtener el directorio de datos de la app");

            // Print app data dir
            println!("App data dir: {}", app_data_dir.display());

            tauri::async_runtime::block_on(async move {
                let pool = init_db(app_data_dir)
                    .await
                    .expect("Error al inicializar la base de datos");

                // Expire overdue memberships on startup
                let _ = sqlx::query(
                    "UPDATE memberships SET status = 'expired' WHERE status = 'active' AND end_date < date('now')",
                )
                .execute(&pool)
                .await;

                app.manage(AppState { db: pool });
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            auth::commands::has_users,
            auth::commands::setup_admin,
            auth::commands::login,
            auth::commands::logout,
            auth::commands::validate_session,
            users::commands::list_users,
            users::commands::create_user,
            users::commands::update_user,
            users::commands::deactivate_user,
            members::commands::list_members,
            members::commands::get_member,
            members::commands::search_members,
            members::commands::create_member,
            members::commands::update_member,
            members::commands::deactivate_member,
            members::commands::regenerate_access_code,
            memberships::commands::list_membership_types,
            memberships::commands::update_membership_type_price,
            memberships::commands::assign_membership,
            memberships::commands::renew_membership,
            memberships::commands::get_member_memberships,
            memberships::commands::get_expiring_memberships,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
