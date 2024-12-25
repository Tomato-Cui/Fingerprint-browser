use crate::response::AppResponse;
use models::environment_fingerprint::EnvironmentFingerprint;
use serde_json::Value;

use super::user::get_user_id;

#[tauri::command]
pub async fn environment_fingerprint_query_id(
    id: u32,
) -> Result<AppResponse<EnvironmentFingerprint>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    Ok(match services::environment_fingerprint::query_by_id(&user_uuid, id).await {
        Ok(data) => AppResponse::<EnvironmentFingerprint>::success(success_msg, Some(data)),
        Err(r) => AppResponse::<EnvironmentFingerprint>::fail(warn_msg(r.to_string())),
    })
}

#[tauri::command]
pub async fn environment_fingerprint_query(
    page_num: u32,
    page_size: u32,
) -> Result<AppResponse<Value>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    match services::environment_fingerprint::query(&user_uuid, page_num, page_size).await {
        Ok(data) => Ok(AppResponse::<Value>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<Value>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn environment_fingerprint_create(
    payload: EnvironmentFingerprint,
) -> Result<AppResponse<bool>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
        Some(format!("创建失败: {}", v))
    });

    match services::environment_fingerprint::create(&user_uuid, &payload).await {
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
pub async fn environment_fingerprint_modify(
    id: u32,
    mut payload: EnvironmentFingerprint,
) -> Result<AppResponse<bool>, tauri::Error> {
    let user_uuid = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
        Some(format!("更新失败: {}", v))
    });

    payload.id = Some(id as i32);
    Ok(
        match services::environment_fingerprint::modify(&user_uuid, id, &payload).await {
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
pub async fn environment_fingerprint_delete(id: u32) -> Result<AppResponse<bool>, tauri::Error> {
    let user_uuid = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
        Some(format!("删除失败: {}", v))
    });

    Ok(match services::environment_fingerprint::delete(&user_uuid, id).await {
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