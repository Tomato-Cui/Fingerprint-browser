use crate::response::AppResponse;
use models::environment::Environment;
use serde_json::Value;

use super::user::get_user_id;

#[tauri::command]
pub async fn environment_trash_query_id(
    environment_uuid: String,
) -> Result<AppResponse<Environment>, tauri::Error> {
    let _ = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    Ok(
        match services::environment_trash::query_by_environment_uuid(&environment_uuid).await {
            Ok(data) => AppResponse::<Environment>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Environment>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn environment_trash_query(
    page_num: u32,
    page_size: u32,
) -> Result<AppResponse<Value>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    match services::environment_trash::query(&user_uuid, page_num, page_size).await {
        Ok(data) => Ok(AppResponse::<Value>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<Value>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn environment_trash_recover(
    environment_uuid: String,
) -> Result<AppResponse<bool>, tauri::Error> {
    let user_uuid = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("恢复成功".to_string()), |v| {
        Some(format!("恢复失败: {}", v))
    });

    Ok(
        match services::environment_trash::recover(&user_uuid, &environment_uuid).await {
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
pub async fn environment_trash_recovers(
    environment_uuids: Vec<String>,
) -> Result<AppResponse<bool>, tauri::Error> {
    let user_uuid = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("恢复成功".to_string()), |v| {
        Some(format!("恢复失败: {}", v))
    });

    Ok(
        match services::environment_trash::recovers(
            &user_uuid,
            environment_uuids.iter().map(|v| v.as_str()).collect(),
        )
        .await
        {
            Ok(data) => AppResponse::<bool>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn environment_trash_recover_all() -> Result<AppResponse<bool>, tauri::Error> {
    let user_uuid = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("恢复成功".to_string()), |v| {
        Some(format!("恢复失败: {}", v))
    });

    Ok(
        match services::environment_trash::recover_all(&user_uuid).await {
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
pub async fn environment_trash_delete_batch(
    environment_uuids: Vec<String>,
) -> Result<AppResponse<bool>, tauri::Error> {
    let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
        Some(format!("删除失败: {}", v))
    });

    Ok(
        match services::environment_trash::batch_delete_again(
            environment_uuids.iter().map(|v| v.as_str()).collect(),
        )
        .await
        {
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
pub async fn environment_trash_clean() -> Result<AppResponse<bool>, tauri::Error> {
    let user_uuid = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("清空成功".to_string()), |v| {
        Some(format!("清空失败: {}", v))
    });

    Ok(match services::environment_trash::clean(&user_uuid).await {
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
