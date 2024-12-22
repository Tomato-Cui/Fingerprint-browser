use std::sync::Arc;

pub mod command;
pub mod components;
pub mod response;

use command::browser as browser_command;
use command::environment as environment_command;
use command::fingerprint as fingerprint_command;
use command::group as group_command;
use command::proxy as proxy_command;
use command::user as user_command;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            user_command::login,
            user_command::logout,
            user_command::register,
            user_command::reset_password,
            user_command::is_login,
            environment_command::environment_query_id,
            environment_command::environment_query,
            environment_command::environment_query_by_group,
            environment_command::environment_create,
            environment_command::environment_batch,
            environment_command::environment_modify,
            environment_command::move_to_group::environment_move_to_group,
            environment_command::batch_move_to_group::environment_batch_move_to_group,
            environment_command::environment_delete,
            environment_command::environment_batch_delete,
            fingerprint_command::fingerprint_query_id,
            fingerprint_command::fingerprint_query,
            fingerprint_command::fingerprint_create,
            fingerprint_command::fingerprint_modify,
            fingerprint_command::fingerprint_delete,
            group_command::group_query_id,
            group_command::group_query,
            group_command::group_create,
            group_command::group_modify,
            group_command::group_grant_user,
            group_command::group_delete,
            proxy_command::proxies_query_id,
            proxy_command::proxies_query,
            proxy_command::proxies_create,
            proxy_command::proxies_modify,
            proxy_command::proxies_delete,
            browser_command::start,
            browser_command::starts::starts,
            browser_command::stops,
            browser_command::status,
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
