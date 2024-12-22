use crate::response::AppResponse;
use serde_json::{json, Value};

use super::user::get_user_id;

#[tauri::command]
pub async fn proxies_query_id(id: u32) -> Result<AppResponse<Value>, tauri::Error> {
    let user_id = get_user_id().await;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    match services::proxy::query_by_id(user_id, id).await {
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
pub async fn proxies_query(
    page_num: u32,
    page_size: u32,
) -> Result<AppResponse<Value>, tauri::Error> {
    let user_id = get_user_id().await;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    match services::proxy::query(user_id, page_num, page_size).await {
        Ok(data) => Ok(AppResponse::<Value>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<Value>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn proxies_create(
    kind: String,
    value: String,
) -> Result<AppResponse<bool>, tauri::Error> {
    let user_id = get_user_id().await;
    let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
        Some(format!("创建失败: {}", v))
    });

    let proxy = models::proxies::Proxy {
        kind: kind.clone(),
        value: value.clone(),
        ..Default::default()
    };

    match services::proxy::create(user_id, &proxy).await {
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
pub async fn proxies_modify(
    id: u32,
    kind: String,
    value: String,
) -> Result<AppResponse<bool>, tauri::Error> {
    let user_id = get_user_id().await;

    let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
        Some(format!("更新失败: {}", v))
    });
    let proxy = models::proxies::Proxy {
        id: id as i32,
        kind,
        value,
        ..Default::default()
    };

    match services::proxy::modify(user_id, &proxy).await {
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
pub async fn proxies_delete(id: u32) -> Result<AppResponse<bool>, tauri::Error> {
    let user_id = get_user_id().await;

    let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
        Some(format!("删除失败: {}", v))
    });

    match services::proxy::delete(user_id, id).await {
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
