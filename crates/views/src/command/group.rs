use super::user::get_user_id;
use crate::response::AppResponse;
use serde_json::Value;

#[tauri::command]
pub async fn group_query_id(id: u32) -> Result<AppResponse<Value>, tauri::Error> {
    let user_id = get_user_id().await;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    match services::group::query_by_id(id, user_id).await {
        Ok(data) => Ok(AppResponse::<Value>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<Value>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn group_query(
    page_num: u32,
    page_size: u32,
) -> Result<AppResponse<Value>, tauri::Error> {
    let user_id = get_user_id().await;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    match services::group::query(user_id, page_num, page_size).await {
        Ok(data) => Ok(AppResponse::<Value>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<Value>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn group_create(
    name: String,
    description: Option<String>,
) -> Result<AppResponse<bool>, tauri::Error> {
    let user_id = get_user_id().await;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    let group = models::group::Group {
        name: name.clone(),
        description: description.clone(),
        owner_id: user_id as i32,
        ..Default::default()
    };
    match services::group::create(user_id, &group).await {
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
pub async fn group_modify(
    id: u32,
    name: String,
    description: Option<String>,
) -> Result<AppResponse<bool>, tauri::Error> {
    let user_id = get_user_id().await;
    let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
        Some(format!("更新失败: {}", v))
    });

    let group = models::group::Group {
        id: id as i32,
        name: name.clone(),
        description: description.clone(),
        ..Default::default()
    };

    match services::group::modify(user_id, &group).await {
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
pub async fn group_grant_user(id: u32) -> Result<AppResponse<bool>, tauri::Error> {
    let user_id = get_user_id().await;
    let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
        Some(format!("更新失败: {}", v))
    });

    match services::group::grant_user(user_id, id).await {
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
pub async fn group_delete(id: u32) -> Result<AppResponse<bool>, tauri::Error> {
    let user_id = get_user_id().await;
    let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
        Some(format!("删除失败: {}", v))
    });

    match services::group::delete(user_id, id).await {
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
