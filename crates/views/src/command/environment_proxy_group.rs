use crate::response::AppResponse;
use models::environment_proxy_group::ProxyGroup;
use serde_json::Value;

use super::user::get_user_id;

#[tauri::command]
pub async fn environment_proxy_group_query_id(
    id: u32,
) -> Result<AppResponse<ProxyGroup>, tauri::Error> {
    let _ = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    Ok(
        match services::environment_proxy_group::query_by_group_id(id).await {
            Ok(data) => AppResponse::<ProxyGroup>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<ProxyGroup>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn environment_proxy_group_query(
    page_num: u32,
    page_size: u32,
) -> Result<AppResponse<Value>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    match services::environment_proxy_group::query(&user_uuid, page_num, page_size).await {
        Ok(data) => Ok(AppResponse::<Value>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<Value>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn environment_proxy_group_create(
    payload: ProxyGroup,
) -> Result<AppResponse<bool>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
        Some(format!("创建失败: {}", v))
    });

    match services::environment_proxy_group::create(&user_uuid, payload).await {
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
pub async fn environment_proxy_group_modify(
    id: u32,
    payload: ProxyGroup,
) -> Result<AppResponse<bool>, tauri::Error> {
    let _ = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
        Some(format!("更新失败: {}", v))
    });

    Ok(
        match services::environment_proxy_group::update(id, payload).await {
            Ok(data) => {
                if data {
                    AppResponse::<bool>::success(success_msg, Some(data))
                } else {
                    AppResponse::<bool>::fail(warn_msg("未知错误".to_string()))
                }
            }
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn environment_proxy_group_delete(id: u32) -> Result<AppResponse<bool>, tauri::Error> {
    let _ = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
        Some(format!("删除失败: {}", v))
    });

    Ok(match services::environment_proxy_group::delete(id).await {
        Ok(data) => {
            if data {
                AppResponse::<bool>::success(success_msg, Some(data))
            } else {
                AppResponse::<bool>::fail(warn_msg("未知错误".to_string()))
            }
        }
        Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
    })
}
