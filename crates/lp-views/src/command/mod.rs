pub mod browser;
pub mod environment;
pub mod environment_account;
pub mod environment_cookie;
pub mod environment_fingerprint;
pub mod environment_group;
pub mod environment_proxies;
pub mod environment_proxy_group;
pub mod environment_tag;
pub mod environment_transfer_history;
pub mod environment_trash;
pub mod extension;
pub mod message;
pub mod operation_log;
pub mod os;
pub mod team;
pub mod team_group;
pub mod updator;
pub mod user;

pub use browser as browser_command;
pub use environment as environment_command;
pub use environment_account as environment_account_command;
pub use environment_cookie as environment_cookie_command;
pub use environment_fingerprint as environment_fingerprint_command;
pub use environment_group as environment_group_command;
pub use environment_proxies as environment_proxies_command;
pub use environment_proxy_group as environment_proxy_group_command;
pub use environment_tag as environment_tag_command;
pub use environment_transfer_history as environment_transfer_history_command;
pub use environment_trash as environment_trash_command;
pub use extension as extension_command;
pub use message as message_command;
pub use operation_log as operation_log_command;
pub use team as team_command;
pub use team_group as team_group_command;
pub use user as user_command;

use lp_services::command::Actuator;
use serde_json::Value;
use tauri::ipc::Invoke;
use tauri::{AppHandle, Emitter};

pub async fn get_user_id() -> Result<String, anyhow::Error> {
    let token = lp_states::auth::get_token().await;
    if let Some(token_str) = token {
        match lp_commons::encryption::verify_token(&token_str) {
            Ok(user_uuid) => Ok(user_uuid),
            Err(_e) => Err(anyhow::anyhow!("token 异常")),
        }
    } else {
        Err(anyhow::anyhow!("用户处于退出状态"))
    }
}

#[tauri::command]
pub async fn ip_info(
    kind: &str,
    host: &str,
    port: &str,
    username: Option<&str>,
    password: Option<&str>,
) -> Result<crate::response::AppResponse<Value>, tauri::Error> {
    let (success_msg, warn_msg) = (Some("获取成功".to_string()), |v| {
        Some(format!("获取失败: {}", v))
    });

    Ok(
        match lp_cores::requests::iprust_info(kind, host, port, username, password).await {
            Ok(ok) => crate::response::AppResponse::<Value>::success(success_msg, Some(ok)),
            Err(r) => crate::response::AppResponse::<Value>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn init_porcessor(
    app: AppHandle,
) -> Result<crate::response::AppResponse<bool>, tauri::Error> {
    tokio::spawn(async move {
        Actuator::listen_events(|environment_uuid: &str| {
            if let Err(e) = app.emit("environment_close", environment_uuid) {
                eprintln!("Failed to emit environment_close event: {}", e);
            }
        })
        .await;
    });

    Ok(crate::response::AppResponse::<bool>::success(
        Some("init process finish.".to_string()),
        Some(true),
    ))
}

#[tauri::command]
pub async fn init_command(
    app: AppHandle,
) -> Result<crate::response::AppResponse<bool>, tauri::Error> {
    let _ = app.emit("init_command", ());
    Ok(crate::response::AppResponse::<bool>::success(
        Some("init command finish.".to_string()),
        Some(true),
    ))
}

pub fn register_handles() -> impl Fn(Invoke<tauri::Wry>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
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
        environment_tag_command::environment_tag_query_id,
        environment_tag_command::environment_tag_query,
        environment_tag_command::environment_tag_create,
        environment_tag_command::environment_tag_modify,
        environment_tag_command::environment_tag_delete,
        environment_proxies_command::environment_proxies_query_id,
        environment_proxies_command::environment_proxies_query,
        environment_proxies_command::environment_proxies_query_by_group,
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
        environment_command::environment_simple_create,
        environment_command::environment_advanced_create,
        environment_command::environment_modify_info,
        environment_command::environment_modify_basic_info,
        environment_command::environment_move_to_group,
        environment_command::environment_batch_move_to_group,
        environment_command::environment_batch_move_to_tag,
        environment_command::environment_delete,
        environment_command::environment_batch_delete,
        environment_command::environment_modify_proxy,
        team_command::team_is_leader,
        team_command::team_query_id,
        team_command::team_query,
        team_command::team_query_name,
        team_command::query_current_team_info,
        team_command::blocked,
        team_command::un_blocked,
        team_command::query_team_all_user,
        team_command::query_team_all_blocked_user,
        team_command::query_team_group_all_user,
        team_command::team_create,
        team_command::team_modify,
        team_command::switch_team,
        team_command::remove_current_user,
        team_command::team_modify_team_user_info,
        team_command::team_delete,
        team_group_command::team_group_query_id,
        team_group_command::team_group_query_all,
        message_command::user_receive_query,
        message_command::team_receive_query,
        message_command::team_send,
        message_command::user_send,
        message_command::reject,
        message_command::team_allow,
        message_command::user_allow,
        user_command::login,
        user_command::logout,
        user_command::register,
        user_command::user_query_search_by_email,
        user_command::reset_password,
        user_command::is_login,
        user_command::register_send,
        user_command::reset_password_send,
        extension_command::extension_info_by_chrome_store_url,
        extension_command::extension_create,
        extension_command::extension_user_create,
        extension_command::extension_team_create,
        extension_command::extension_query_by_team,
        extension_command::extension_query_by_user,
        extension_command::extension_query_by_environment,
        extension_command::extension_query,
        extension_command::extension_environment_use_extension,
        extension_command::extension_environment_remove_extension,
        extension_command::extension_update,
        extension_command::user_toggle_extension,
        extension_command::extension_delete_by_uuid,
        extension_command::extension_remove_by_user_uuid,
        browser_command::browser_start,
        browser_command::starts::browser_starts,
        browser_command::browser_stops,
        browser_command::browser_status,
        operation_log_command::opeartion_query,
        operation_log_command::opeartion_query_by_team,
        os::platform,
        ip_info,
        init_command,
        init_porcessor,
        updator::app_updates::fetch_update,
        updator::app_updates::install_update,
    ]
}
