use std::collections::HashMap;

use crate::response::AppResponse;
use models::environment_transfer_history::EnvironmentTransferHistory;
use serde_json::Value;

use super::user::get_user_id;

#[tauri::command]
pub async fn environment_transfer_history_query_id(
    environment_uuid: String,
) -> Result<AppResponse<EnvironmentTransferHistory>, tauri::Error> {
    let _ = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    Ok(
        match services::environment_transfer_history::query_by_id(&environment_uuid).await {
            Ok(data) => AppResponse::<EnvironmentTransferHistory>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<EnvironmentTransferHistory>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn environment_transfer_history_query(
    page_num: u32,
    page_size: u32,
) -> Result<AppResponse<Value>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    match services::environment_transfer_history::query(&user_uuid, page_num, page_size).await {
        Ok(data) => Ok(AppResponse::<Value>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<Value>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn environment_transfer_history_batch_create(
    current_user_email: &str,
    environment_uuids: Vec<String>,
) -> Result<AppResponse<HashMap<String, bool>>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
        Some(format!("创建失败: {}", v))
    });

    match services::environment_transfer_history::create(
        &user_uuid,
        current_user_email,
        environment_uuids,
    )
    .await
    {
        Ok(data) => Ok(AppResponse::<HashMap<String, bool>>::success(
            success_msg,
            Some(data),
        )),
        Err(r) => Ok(AppResponse::<HashMap<String, bool>>::fail(warn_msg(
            r.to_string(),
        ))),
    }
}

#[tauri::command]
pub async fn environment_transfer_history_delete(
    environment_uuid: String,
) -> Result<AppResponse<bool>, tauri::Error> {
    let user_uuid = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
        Some(format!("删除失败: {}", v))
    });

    Ok(
        match services::environment_transfer_history::delete(&user_uuid, &environment_uuid).await {
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
