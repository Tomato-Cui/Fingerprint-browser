use crate::response::AppResponse;
use cores::request::JsonRespnse;
use serde_json::Value;

#[tauri::command]
pub async fn extension_info_by_chrome_store_url(
    url: &str,
) -> Result<AppResponse<Value>, tauri::Error> {
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
pub async fn extension_create(
    extension: models::extension::Extension,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::extension::create(extension).await?)
}

#[tauri::command]
pub async fn extension_user_create(extension_uuid: String) -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::extension::user_create(&extension_uuid).await?)
}

#[tauri::command]
pub async fn extension_team_create(
    team_id: String,
    extension_uuid: String,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::extension::team_create(&team_id, &extension_uuid).await?)
}

#[tauri::command]
pub async fn extension_query_by_team(
    team_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::extension::query_by_team(team_id, page_num, page_size).await?)
}

#[tauri::command]
pub async fn extension_query_by_user(
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::extension::query_by_user(page_num, page_size).await?)
}

#[tauri::command]
pub async fn extension_query_by_environment(
    environment_uuid: String,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::extension::query_by_environment(
        &environment_uuid,
        page_num,
        page_size,
    )
    .await?)
}

#[tauri::command]
pub async fn extension_query(page_num: u32, page_size: u32) -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::extension::query(page_num, page_size).await?)
}

#[tauri::command]
pub async fn extension_environment_use_extension(
    extension_uuid: String,
    environment_uuids: Vec<String>,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(
        services_remote::requests::extension::environment_use_extension(
            &extension_uuid,
            environment_uuids,
        )
        .await?,
    )
}

#[tauri::command]
pub async fn extension_environment_remove_extension(
    extension_uuid: String,
    environment_uuid: String,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(
        services_remote::requests::extension::environment_remove_extension(
            &extension_uuid,
            &environment_uuid,
        )
        .await?,
    )
}

#[tauri::command]
pub async fn extension_update(
    extension_uuid: String,
    extension: models::extension::Extension,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::extension::update(&extension_uuid, extension).await?)
}

#[tauri::command]
pub async fn user_toggle_extension(
    extension_uuid: String,
    open: bool,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::extension::user_toggle_extension(&extension_uuid, open).await?)
}

#[tauri::command]
pub async fn extension_delete_by_uuid(extension_uuid: String) -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::extension::delete_by_uuid(&extension_uuid).await?)
}

#[tauri::command]
pub async fn extension_remove_by_user_uuid(
    extension_uuid: String,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::extension::remove_by_user_uuid(&extension_uuid).await?)
}
