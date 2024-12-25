use crate::response::AppResponse;
use serde_json::{json, Value};

use super::user::get_user_id;

#[tauri::command]
pub async fn environment_proxies_query_id(id: u32) -> Result<AppResponse<Value>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    match services::environment_proxy::query_by_id(&user_uuid, id).await {
        Ok(data) => Ok(AppResponse::<Value>::success(
            success_msg,
            Some(json! ({
                "data": data,
            })),
        )),
        Err(r) => Ok(AppResponse::<Value>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn environment_proxies_query(
    page_num: u32,
    page_size: u32,
) -> Result<AppResponse<Value>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    match services::environment_proxy::query_by_uuid(&user_uuid, page_num, page_size).await {
        Ok(data) => Ok(AppResponse::<Value>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<Value>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn environment_proxies_create(
    kind: String,
    host: String,
    port: String,
    username: Option<String>,
    password: Option<String>,
) -> Result<AppResponse<bool>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
        Some(format!("创建失败: {}", v))
    });

    let proxy = models::environment_proxies::Proxy {
        kind: kind.clone(),
        host: host.clone(),
        port: port.clone(),
        username: username.clone(),
        password: password.clone(),
        user_uuid: Some(user_uuid.clone()),
        ..Default::default()
    };

    match services::environment_proxy::create(&user_uuid, proxy).await {
        Ok(data) => {
            if data {
                Ok(AppResponse::<bool>::success(success_msg, Some(data)))
            } else {
                Ok(AppResponse::<bool>::fail(warn_msg("未知错误".to_string())))
            }
        }
        Err(r) => Ok(AppResponse::<bool>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn environment_proxies_modify(
    id: u32,
    kind: String,
    host: String,
    port: String,
    username: Option<String>,
    password: Option<String>,
) -> Result<AppResponse<bool>, tauri::Error> {
    let user_uuid = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
        Some(format!("更新失败: {}", v))
    });

    let proxy = models::environment_proxies::Proxy {
        id: id as i32,
        kind: kind.clone(),
        host: host.clone(),
        port: port.clone(),
        username: username.clone(),
        password: password.clone(),
        user_uuid: Some(user_uuid.clone()),
        ..Default::default()
    };

    match services::environment_proxy::update(&user_uuid, proxy).await {
        Ok(data) => {
            if data {
                Ok(AppResponse::<bool>::success(success_msg, Some(data)))
            } else {
                Ok(AppResponse::<bool>::fail(warn_msg("未知错误".to_string())))
            }
        }
        Err(r) => Ok(AppResponse::<bool>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn environment_proxies_delete(id: u32) -> Result<AppResponse<bool>, tauri::Error> {
    let user_uuid = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
        Some(format!("删除失败: {}", v))
    });

    match services::environment_proxy::delete(&user_uuid, id).await {
        Ok(data) => {
            if data {
                Ok(AppResponse::<bool>::success(success_msg, Some(data)))
            } else {
                Ok(AppResponse::<bool>::fail(warn_msg("未知错误".to_string())))
            }
        }
        Err(r) => Ok(AppResponse::<bool>::fail(warn_msg(r.to_string()))),
    }
}
