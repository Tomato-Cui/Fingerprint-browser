use components::windows::tray;
use tauri::AppHandle;
use tauri::Manager;
use tauri::RunEvent;
use tauri::Window;
use tauri::WindowEvent;

pub mod command;
pub mod components;
pub mod response;

use command::browser as browser_command;
use command::environment as environment_command;
use command::environment_account as environment_account_command;
use command::environment_cookie as environment_cookie_command;
use command::environment_fingerprint as environment_fingerprint_command;
use command::environment_group as environment_group_command;
use command::environment_proxies as environment_proxies_command;
use command::environment_proxy_group as environment_proxy_group_command;
use command::environment_transfer_history as environment_transfer_history_command;
use command::environment_trash as environment_trash_command;
use command::extension as extension_command;
use command::team as team_command;
use command::team_group as team_group_command;
use command::user as user_command;
use command::user_team_temp as user_team_temp_command;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            environment_account_command::environment_account_query_id,
            environment_account_command::environment_account_query,
            environment_account_command::environment_account_query_current,
            environment_account_command::environment_account_create,
            environment_account_command::environment_account_modify,
            environment_account_command::environment_account_delete,
            environment_account_command::environment_account_batch_delete,
            environment_cookie_command::environment_cookie_query_environment_uuid,
            environment_cookie_command::environment_cookie_create,
            environment_cookie_command::environment_cookie_modify,
            environment_cookie_command::environment_cookie_delete,
            environment_fingerprint_command::environment_fingerprint_query_id,
            environment_fingerprint_command::environment_fingerprint_query,
            environment_fingerprint_command::environment_fingerprint_create,
            environment_fingerprint_command::environment_fingerprint_modify,
            environment_fingerprint_command::environment_fingerprint_delete,
            environment_group_command::environment_group_query_id,
            environment_group_command::environment_group_query,
            environment_group_command::environment_group_create,
            environment_group_command::environment_group_modify,
            environment_group_command::environment_group_delete,
            environment_proxies_command::environment_proxies_query_id,
            environment_proxies_command::environment_proxies_query,
            environment_proxies_command::environment_proxies_create,
            environment_proxies_command::environment_proxies_modify,
            environment_proxies_command::environment_proxies_delete,
            environment_proxies_command::environment_proxies_batch_delete,
            environment_proxy_group_command::environment_proxy_group_query_id,
            environment_proxy_group_command::environment_proxy_group_query,
            environment_proxy_group_command::environment_proxy_group_create,
            environment_proxy_group_command::environment_proxy_group_modify,
            environment_proxy_group_command::environment_proxy_group_delete,
            environment_transfer_history_command::environment_transfer_history_query_id,
            environment_transfer_history_command::environment_transfer_history_query,
            environment_transfer_history_command::environment_transfer_history_batch_create,
            environment_transfer_history_command::environment_transfer_history_delete,
            environment_trash_command::environment_trash_query_id,
            environment_trash_command::environment_trash_query,
            environment_trash_command::environment_trash_recover,
            environment_trash_command::environment_trash_recovers,
            environment_trash_command::environment_trash_recover_all,
            environment_trash_command::environment_trash_delete_batch,
            environment_trash_command::environment_trash_clean,
            environment_command::environment_query_id,
            environment_command::environment_query,
            environment_command::environment_query_by_group,
            environment_command::environment_query_by_team,
            environment_command::environment_query_by_extension,
            environment_command::environment_create,
            environment_command::environment_detail_create,
            environment_command::environment_batch_create,
            environment_command::environment_modify_info,
            environment_command::environment_modify_basic_info,
            environment_command::environment_move_to_group,
            environment_command::environment_batch_move_to_group,
            environment_command::environment_delete,
            environment_command::environment_batch_delete,
            team_command::team_query_id,
            team_command::team_query,
            team_command::query_current_team_info,
            team_command::blocked,
            team_command::un_blocked,
            team_command::query_team_all_user,
            team_command::query_team_all_blocked_user,
            team_command::query_team_group_all_user,
            team_command::team_create,
            team_command::team_modify,
            team_command::remove_current_user,
            team_command::team_modify_team_user_info,
            team_command::team_delete,
            team_group_command::team_group_query_id,
            team_group_command::team_group_query_all,
            user_team_temp_command::user_receive_query,
            user_team_temp_command::team_receive_query,
            user_team_temp_command::team_send,
            user_team_temp_command::user_send,
            user_team_temp_command::reject,
            user_team_temp_command::team_allow,
            user_team_temp_command::user_allow,
            user_command::login,
            user_command::logout,
            user_command::register,
            user_command::reset_password,
            user_command::is_login,
            user_command::register_send,
            extension_command::extension_info_by_chrome_store_url,
            extension_command::extension_create,
            extension_command::extension_user_create,
            extension_command::extension_team_create,
            extension_command::extension_query_by_team,
            extension_command::extension_query_by_user,
            extension_command::extension_query_by_environment,
            extension_command::extension_query,
            extension_command::extension_environmnet_use_extension,
            extension_command::extension_environmnet_remove_extension,
            extension_command::extension_update,
            extension_command::user_toggle_extension,
            extension_command::extension_delete_by_uuid,
            extension_command::extension_remove_by_user_uuid,
            browser_command::browser_start,
            browser_command::starts::browser_starts,
            browser_command::browser_stops,
            browser_command::browser_status,
            command::os::platform,
        ])
        .setup(|app| {
            #[cfg(desktop)]
            app.handle()
                .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }))
                .unwrap();

            #[cfg(debug_assertions)]
            app.handle()
                .plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )
                .unwrap();

            tray::menu(app)?;
            Ok(())
        })
        .on_window_event(window_event_handle)
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(run_event_handle);
}

fn window_event_handle(window: &Window, event: &WindowEvent) {
    match event {
        tauri::WindowEvent::CloseRequested { api, .. } => {
            api.prevent_close();
            let _ = window.hide();
        }
        _ => {}
    }
}

fn run_event_handle(_app_handle: &AppHandle, event: RunEvent) {
    match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    }
}
