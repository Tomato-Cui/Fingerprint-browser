use crate::response::AppResponse;
use models::{environment::Environment, environment_proxies::Proxy};
use serde_json::Value;

use super::user::get_user_id;

#[tauri::command]
pub async fn environment_query_id(
    environment_uuid: String,
) -> Result<AppResponse<Value>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    Ok(
        match services::environment::query_environment_details(&user_uuid, &environment_uuid).await
        {
            Ok(data) => AppResponse::<Value>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        },
    )
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
    id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<AppResponse<Value>, tauri::Error> {
    let _ = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    match services::environment::query_by_group_id(id, page_num, page_size).await {
        Ok(data) => Ok(AppResponse::<Value>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<Value>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn environment_query_by_team(
    id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<AppResponse<Value>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    match services::environment::query_by_team_id(&user_uuid, id, page_num, page_size).await {
        Ok(data) => Ok(AppResponse::<Value>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<Value>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn environment_query_by_extension(
    extension_uuid: &str,
    page_num: u32,
    page_size: u32,
) -> Result<AppResponse<Value>, tauri::Error> {
    let _ = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    Ok(
        match services::extension::query_environmnets_by_extension_uuid(
            &extension_uuid,
            page_num,
            page_size,
        )
        .await
        {
            Ok(ok) => AppResponse::<Value>::success(success_msg, Some(ok)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn environment_detail_create(
    payload: models::environment::EnvironmentInfo,
) -> Result<AppResponse<bool>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
        Some(format!("创建失败: {}", v))
    });

    match services::environment::create_and_other_info(&user_uuid, payload).await {
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
pub async fn environment_create(
    environment_name: String,
) -> Result<AppResponse<bool>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
        Some(format!("创建失败: {}", v))
    });

    match services::environment::create(&user_uuid, environment_name).await {
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
pub async fn environment_batch_create(
    environment_names: Vec<String>,
) -> Result<AppResponse<Vec<Value>>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
        Some(format!("创建失败: {}", v))
    });

    match services::environment::create_batch(&user_uuid, environment_names).await {
        Ok(data) => Ok(AppResponse::<Vec<Value>>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<Vec<Value>>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn environment_modify_info(
    environment_uuid: &str,
    payload: models::environment::EnvironmentInfo,
) -> Result<AppResponse<bool>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
        Some(format!("更新失败: {}", v))
    });

    Ok(
        match services::environment::modify_info(&user_uuid, environment_uuid, payload).await {
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
pub async fn environment_modify_proxy(
    environment_uuid: String,
    payload: Proxy,
) -> Result<AppResponse<bool>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
        Some(format!("更新失败: {}", v))
    });

    Ok(
        match services::environment_proxy::update_proxy(&user_uuid, &environment_uuid, payload)
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
pub async fn environment_modify_basic_info(
    environment_uuid: String,
    payload: Environment,
) -> Result<AppResponse<bool>, tauri::Error> {
    let _ = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
        Some(format!("更新失败: {}", v))
    });

    Ok(
        match services::environment::modify_basic_info(
            &environment_uuid,
            &payload.name,
            payload.description,
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
pub async fn environment_move_to_group(
    environment_uuid: String,
    group_id: u32,
) -> Result<AppResponse<bool>, tauri::Error> {
    let _ = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
        Some(format!("更新失败: {}", v))
    });

    Ok(
        match services::environment::move_to_group(&environment_uuid, group_id).await {
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
pub async fn environment_batch_move_to_group(
    environment_uuids: Vec<String>,
    group_id: u32,
) -> Result<AppResponse<Vec<Value>>, tauri::Error> {
    let _ = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
        Some(format!("更新失败: {}", v))
    });

    match services::environment::batch_move_to_group(environment_uuids, group_id).await {
        Ok(data) => Ok(AppResponse::<Vec<Value>>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<Vec<Value>>::fail(warn_msg(r.to_string()))),
    }
}

#[tauri::command]
pub async fn environment_delete(
    environment_uuid: String,
) -> Result<AppResponse<bool>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
        Some(format!("删除失败: {}", v))
    });

    Ok(
        match services::environment::delete(&user_uuid, &environment_uuid).await {
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
pub async fn environment_batch_delete(
    environment_uuids: Vec<String>,
) -> Result<AppResponse<Vec<bool>>, tauri::Error> {
    let user_uuid = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
        Some(format!("删除失败: {}", v))
    });

    match services::environment::batch_delete(&user_uuid, environment_uuids).await {
        Ok(data) => Ok(AppResponse::<Vec<bool>>::success(success_msg, Some(data))),
        Err(r) => Ok(AppResponse::<Vec<bool>>::fail(warn_msg(r.to_string()))),
    }
}
