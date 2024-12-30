use crate::response::AppResponse;
use models::extension;
use serde_json::Value;

use super::user::get_user_id;

#[tauri::command]
pub async fn extension_info_by_chrome_store_url(
    url: &str,
) -> Result<AppResponse<Value>, tauri::Error> {
    let _ = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("获取成功".to_string()), |v| {
        Some(format!("获取失败: {}", v))
    });

    Ok(
        match cores::extensions::chrome_extension_scrapy::extension_detail_by_url(url).await {
            Ok(ok) => AppResponse::<Value>::success(success_msg, Some(ok)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn extension_user_create(
    extension: extension::Extension,
) -> Result<AppResponse<bool>, tauri::Error> {
    let user_uuid = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
        Some(format!("创建失败: {}", v))
    });

    Ok(
        match services::extension::user_insert(&user_uuid, extension).await {
            Ok(ok) => AppResponse::<bool>::success(success_msg, Some(ok)),
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn extension_team_create(
    team_id: String,
    extension: extension::Extension,
) -> Result<AppResponse<bool>, tauri::Error> {
    let _ = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
        Some(format!("创建失败: {}", v))
    });

    Ok(
        match services::extension::team_insert(&team_id, extension).await {
            Ok(ok) => AppResponse::<bool>::success(success_msg, Some(ok)),
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn extension_query_by_team(
    team_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<AppResponse<Value>, tauri::Error> {
    let _ = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    Ok(
        match services::extension::query_by_team_id(team_id, page_num, page_size).await {
            Ok(ok) => AppResponse::<Value>::success(success_msg, Some(ok)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn extension_query_by_user(
    page_num: u32,
    page_size: u32,
) -> Result<AppResponse<Value>, tauri::Error> {
    let user_uuid = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    Ok(
        match services::extension::query_by_user_uuid(&user_uuid, page_num, page_size).await {
            Ok(ok) => AppResponse::<Value>::success(success_msg, Some(ok)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn extension_query_by_environment(
    environmnet_uuid: &str,
    page_num: u32,
    page_size: u32,
) -> Result<AppResponse<Value>, tauri::Error> {
    let _ = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    Ok(
        match services::extension::query_by_environmnet_uuid(&environmnet_uuid, page_num, page_size)
            .await
        {
            Ok(ok) => AppResponse::<Value>::success(success_msg, Some(ok)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn extension_query(
    page_num: u32,
    page_size: u32,
) -> Result<AppResponse<Value>, tauri::Error> {
    let _ = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    Ok(
        match services::extension::query(page_num, page_size).await {
            Ok(ok) => AppResponse::<Value>::success(success_msg, Some(ok)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn extension_environmnet_use_extension(
    extension_uuid: String,
    environment_uuid: String,
) -> Result<AppResponse<bool>, tauri::Error> {
    let _ = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
        Some(format!("更新失败: {}", v))
    });

    Ok(
        match services::extension::environmnet_use_extension(&extension_uuid, &environment_uuid)
            .await
        {
            Ok(ok) => AppResponse::<bool>::success(success_msg, Some(ok)),
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn extension_update(
    extension_uuid: String,
    extension: extension::Extension,
) -> Result<AppResponse<bool>, tauri::Error> {
    let _ = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
        Some(format!("更新失败: {}", v))
    });

    Ok(
        match services::extension::update(&extension_uuid, extension).await {
            Ok(ok) => AppResponse::<bool>::success(success_msg, Some(ok)),
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn extension_delete_by_uuid(
    extension_uuid: String,
) -> Result<AppResponse<bool>, tauri::Error> {
    let _ = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
        Some(format!("删除失败: {}", v))
    });

    Ok(match services::extension::delete(&extension_uuid).await {
        Ok(ok) => AppResponse::<bool>::success(success_msg, Some(ok)),
        Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
    })
}
