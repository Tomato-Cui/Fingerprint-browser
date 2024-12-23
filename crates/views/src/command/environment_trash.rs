use serde_json::{json, Value};

use crate::response::AppResponse;

use super::user::get_user_id;

#[tauri::command]
pub async fn environment_trash_query_id(id: u32) -> Result<AppResponse<Value>, tauri::Error> {
    let user_id = get_user_id().await;

    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    Ok(
        match services::environment::query_by_id(Some(user_id), None, id).await {
            Ok(data) => AppResponse::<Value>::success(
                success_msg,
                Some(json!({
                    "data": data,
                })),
            ),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn environment_trash_query(
    page_num: u32,
    page_size: u32,
) -> Result<AppResponse<Value>, tauri::Error> {
    let user_id = get_user_id().await;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    Ok(
        match services::environment::query(user_id, page_num, page_size).await {
            Ok(data) => AppResponse::<Value>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn environment_trash_recover(id: u32) -> Result<AppResponse<bool>, tauri::Error> {
    let user_id = get_user_id().await;
    let (success_msg, warn_msg) = (Some("恢复成功".to_string()), |v| {
        Some(format!("恢复失败: {}", v))
    });

    Ok(
        match services::environment_trash::recover(user_id, id).await {
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
    environment_ids: Vec<u32>,
) -> Result<AppResponse<Vec<Value>>, tauri::Error> {
    let user_id = get_user_id().await;
    let (success_msg, warn_msg) = (Some("恢复成功".to_string()), |v| {
        Some(format!("恢复成功: {}", v))
    });

    Ok(
        match services::environment_trash::recovers(user_id, environment_ids.clone()).await {
            Ok(data) => AppResponse::<Vec<Value>>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Vec<Value>>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn environment_trash_recover_all() -> Result<AppResponse<bool>, tauri::Error> {
    let user_id = get_user_id().await;
    let (success_msg, warn_msg) = (Some("恢复成功".to_string()), |v| {
        Some(format!("恢复成功: {}", v))
    });

    Ok(
        match services::environment_trash::recover_all(user_id).await {
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
pub async fn environment_trash_delete_again(id: u32) -> Result<AppResponse<bool>, tauri::Error> {
    let user_id = get_user_id().await;
    let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
        Some(format!("删除成功: {}", v))
    });

    Ok(
        match services::environment_trash::delete_again(user_id, id).await {
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
    let user_id = get_user_id().await;
    let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
        Some(format!("删除成功: {}", v))
    });

    Ok(match services::environment_trash::clean(user_id).await {
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
