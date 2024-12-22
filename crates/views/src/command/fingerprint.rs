use super::user::get_user_id;
use crate::response::AppResponse;
use serde_json::{json, Value};

#[tauri::command]
pub async fn fingerprint_query_id(id: u32) -> Result<AppResponse<Value>, tauri::Error> {
    let user_id = get_user_id().await;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    match services::fingerprint::query_by_id(user_id, id).await {
        Ok(data) => Ok(AppResponse::<Value>::success(
            success_msg,
            Some(json!({
                "data": data,
            })),
        )),
        Err(r) => Ok(AppResponse::<Value>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn fingerprint_query(
    page_num: u32,
    page_size: u32,
) -> Result<AppResponse<Value>, tauri::Error> {
    let user_id = get_user_id().await;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    match services::fingerprint::query(user_id, page_num, page_size).await {
        Ok(data) => Ok(AppResponse::<Value>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<Value>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn fingerprint_create(
    payload: models::fingerprint::Fingerprint,
) -> Result<AppResponse<bool>, tauri::Error> {
    let user_id = get_user_id().await;
    let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
        Some(format!("创建失败: {}", v))
    });

    match services::fingerprint::create(user_id, &payload).await {
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
pub async fn fingerprint_modify(
    id: u32,
    mut payload: models::fingerprint::Fingerprint,
) -> Result<AppResponse<bool>, tauri::Error> {
    let user_id = get_user_id().await;
    let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
        Some(format!("更新失败: {}", v))
    });
    payload.id = Some(id as i32);

    match services::fingerprint::modify(user_id, &payload).await {
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
pub async fn fingerprint_delete(id: u32) -> Result<AppResponse<bool>, tauri::Error> {
    let user_id = get_user_id().await;
    let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
        Some(format!("删除失败: {}", v))
    });

    match services::fingerprint::delete(user_id, id).await {
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
