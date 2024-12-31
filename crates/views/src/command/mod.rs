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

use serde_json::Value;

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
