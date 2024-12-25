use serde_json::Value;

use crate::response::AppResponse;

use super::user::get_user_id;

#[tauri::command]
pub async fn environment_trash_query(
    page_num: u32,
    page_size: u32,
) -> Result<AppResponse<Value>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    Ok(
        match services::environment_trash::query(&user_uuid, page_num, page_size).await {
            Ok(data) => AppResponse::<Value>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn environment_trash_recover(
    environment_uuid: &str,
) -> Result<AppResponse<bool>, tauri::Error> {
    let _ = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("恢复成功".to_string()), |v| {
        Some(format!("恢复失败: {}", v))
    });

    Ok(
        match services::environment_trash::recover(environment_uuid).await {
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
    environment_uuids: Vec<&str>,
) -> Result<AppResponse<bool>, tauri::Error> {
    let _ = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("恢复成功".to_string()), |v| {
        Some(format!("恢复成功: {}", v))
    });

    Ok(
        match services::environment_trash::recovers(environment_uuids).await {
            Ok(data) => AppResponse::<bool>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn environment_trash_recover_all() -> Result<AppResponse<bool>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("恢复成功".to_string()), |v| {
        Some(format!("恢复成功: {}", v))
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
pub async fn environment_trash_batch_delete_again(
    environmnet_uuids: Vec<&str>,
) -> Result<AppResponse<bool>, tauri::Error> {
    let _ = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
        Some(format!("删除成功: {}", v))
    });

    Ok(
        match services::environment_trash::delete_again(environmnet_uuids).await {
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
    let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
        Some(format!("删除成功: {}", v))
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
