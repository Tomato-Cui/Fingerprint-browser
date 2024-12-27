use std::sync::Arc;

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
            environment_account_command::environment_account_create,
            environment_account_command::environment_account_modify,
            environment_account_command::environment_account_delete,
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
            team_command::query_team_all_user,
            team_command::query_team_all_blocked_user,
            team_command::query_team_group_all_user,
            team_command::team_create,
            team_command::team_modify,
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
            browser_command::browser_start,
            browser_command::starts::browser_starts,
            browser_command::browser_stops,
            browser_command::browser_status,
            command::os::platform,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    let app_handle = Arc::new(app.handle().clone());

    // use tauri::Manager;
    //     let app_handle_clone = app_handle.clone();
    //     tauri::async_runtime::spawn(async move {
    //         async move {
    //             let cache_dir = app_handle_clone.path().cache_dir().unwrap();
    //             states::tauri::set_app_cache_location(cache_dir).await;
    //         }
    //         .await;
    //     });

    // 如果在调试模式下，初始化日志插件
    if cfg!(debug_assertions) {
        app_handle
            .plugin(
                tauri_plugin_log::Builder::default()
                    .level(log::LevelFilter::Info)
                    .build(),
            )
            .unwrap();
    }

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}
