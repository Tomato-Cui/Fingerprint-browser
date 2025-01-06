pub mod browser;
pub mod environment;
pub mod environment_account;
pub mod environment_cookie;
pub mod environment_fingerprint;
pub mod environment_group;
pub mod environment_proxies;
pub mod environment_proxy_group;
pub mod environment_transfer_history;
pub mod environment_trash;
pub mod extension;
pub mod os;
pub mod team;
pub mod team_group;
pub mod user;
pub mod user_team_temp;

pub use browser as browser_command;
pub use environment as environment_command;
pub use environment_account as environment_account_command;
pub use environment_cookie as environment_cookie_command;
pub use environment_fingerprint as environment_fingerprint_command;
pub use environment_group as environment_group_command;
pub use environment_proxies as environment_proxies_command;
pub use environment_proxy_group as environment_proxy_group_command;
pub use environment_transfer_history as environment_transfer_history_command;
pub use environment_trash as environment_trash_command;
pub use extension as extension_command;
pub use team as team_command;
pub use team_group as team_group_command;
pub use user as user_command;
pub use user_team_temp as user_team_temp_command;

use serde_json::Value;
use services::command::Actuator;
use tauri::{AppHandle, Emitter};

#[tauri::command]
pub async fn ip_info() -> Result<crate::response::AppResponse<Value>, tauri::Error> {
    let (success_msg, warn_msg) = (Some("获取成功".to_string()), |v| {
        Some(format!("获取失败: {}", v))
    });

    Ok(match cores::requests::iprust_info().await {
        Ok(ok) => crate::response::AppResponse::<Value>::success(success_msg, Some(ok)),
        Err(r) => crate::response::AppResponse::<Value>::fail(warn_msg(r.to_string())),
    })
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
