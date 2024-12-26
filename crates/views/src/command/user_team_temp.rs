use crate::response::AppResponse;
use serde_json::Value;

use super::user::get_user_id;

#[tauri::command]
pub async fn user_receive_query(
    page_num: u32,
    page_size: u32,
) -> Result<AppResponse<Value>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    match services::user_team_temp::query_user_apply(&user_uuid, page_num, page_size).await {
        Ok(data) => Ok(AppResponse::<Value>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<Value>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn team_receive_query(
    page_num: u32,
    page_size: u32,
) -> Result<AppResponse<Value>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    match services::user_team_temp::query_team_apply(&user_uuid, page_num, page_size).await {
        Ok(data) => Ok(AppResponse::<Value>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<Value>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn team_send(
    team_id: u32,
    user_uuid: String,
    description: String,
) -> Result<AppResponse<bool>, tauri::Error> {
    let (success_msg, warn_msg) = (Some("发送成功".to_string()), |v| {
        Some(format!("发送失败: {}", v))
    });

    match services::user_team_temp::team_send(&user_uuid, team_id, &description).await {
        Ok(data) => Ok(AppResponse::<bool>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<bool>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn user_send(
    team_id: u32,
    description: String,
) -> Result<AppResponse<bool>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("发送成功".to_string()), |v| {
        Some(format!("发送失败: {}", v))
    });

    match services::user_team_temp::user_send(&user_uuid, team_id, &description).await {
        Ok(data) => Ok(AppResponse::<bool>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<bool>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn reject(id: u32) -> Result<AppResponse<bool>, tauri::Error> {
    let (success_msg, warn_msg) = (Some("拒绝成功".to_string()), |v| {
        Some(format!("拒绝失败: {}", v))
    });

    match services::user_team_temp::delete(id).await {
        Ok(data) => Ok(AppResponse::<bool>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<bool>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn team_allow(
    id: u32,
    user_uuid: String,
    team_id: u32,
) -> Result<AppResponse<bool>, tauri::Error> {
    let (success_msg, warn_msg) = (Some("通过成功".to_string()), |v| {
        Some(format!("通过失败: {}", v))
    });

    match services::user_team_temp::allow(id, &user_uuid, team_id).await {
        Ok(data) => Ok(AppResponse::<bool>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<bool>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn user_allow(id: u32, team_id: u32) -> Result<AppResponse<bool>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("通过成功".to_string()), |v| {
        Some(format!("通过失败: {}", v))
    });

    match services::user_team_temp::allow(id, &user_uuid, team_id).await {
        Ok(data) => Ok(AppResponse::<bool>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<bool>::fail(warn_msg(r.to_string()))),
    }
}
