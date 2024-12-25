use serde_json::{json, Value};

use crate::response::AppResponse;

use super::user::get_user_id;

#[tauri::command]
pub async fn environment_query_id(
    environment_uuid: &str,
) -> Result<AppResponse<Value>, tauri::Error> {
    let _ = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    match services::environment::query_by_uuid(environment_uuid).await {
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
pub async fn environment_query(
    page_num: u32,
    page_size: u32,
) -> Result<AppResponse<Value>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    match services::environment::query(&user_uuid, page_num, page_size).await {
        Ok(data) => Ok(AppResponse::<Value>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<Value>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn environment_query_by_group(
    group_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<AppResponse<Value>, tauri::Error> {
    let _ = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    match services::environment::query_by_group_id(group_id, page_num, page_size).await {
        Ok(data) => Ok(AppResponse::<Value>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<Value>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn environment_create(
    payload: models::environment::Environment,
) -> Result<AppResponse<bool>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
        Some(format!("创建失败: {}", v))
    });

    match services::environment::create(&user_uuid, payload).await {
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
pub async fn environment_batch(
    mut payload: Vec<models::environment::Environment>,
) -> Result<AppResponse<Vec<Value>>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
        Some(format!("创建失败: {}", v))
    });

    match services::environment::create_batch(&user_uuid, payload).await {
        Ok(data) => Ok(AppResponse::<Vec<Value>>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<Vec<Value>>::fail(warn_msg(r.to_string()))),
    }
}

pub mod move_to_group {
    use super::*;

    #[tauri::command]
    pub async fn environment_move_to_group(
        environment_uuid: String,
        group_id: u32,
    ) -> Result<AppResponse<bool>, tauri::Error> {
        let _ = get_user_id().await?;
        let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
            Some(format!("更新失败: {}", v))
        });

        match services::environment::move_to_group(&environment_uuid, group_id).await {
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
}

pub mod batch_move_to_group {
    use super::*;

    #[tauri::command]
    pub async fn environment_batch_move_to_group(
        environment_uuids: Vec<&str>,
        group_id: u32,
    ) -> Result<AppResponse<Vec<Value>>, tauri::Error> {
        let _ = get_user_id().await?;
        let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
            Some(format!("更新失败: {}", v))
        });

        match services::environment::batch_move_to_group(environment_uuids.clone(), group_id).await
        {
            Ok(data) => Ok(AppResponse::<Vec<Value>>::success(success_msg, Some(data))),
            Err(r) => Ok(AppResponse::<Vec<Value>>::fail(warn_msg(r.to_string()))),
        }
    }
}

#[tauri::command]
pub async fn environment_delete(environmnet_uuid: &str) -> Result<AppResponse<bool>, tauri::Error> {
    let user_id = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
        Some(format!("删除失败: {}", v))
    });

    match services::environment::delete(&user_id, environmnet_uuid).await {
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
pub async fn environment_batch_delete(
    environment_uuids: Vec<&str>,
) -> Result<AppResponse<Vec<bool>>, tauri::Error> {
    let user_id = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
        Some(format!("删除失败: {}", v))
    });

    match services::environment::batch_delete(&user_id, environment_uuids).await {
        Ok(data) => Ok(AppResponse::<Vec<bool>>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<Vec<bool>>::fail(warn_msg(r.to_string()))),
    }
}
