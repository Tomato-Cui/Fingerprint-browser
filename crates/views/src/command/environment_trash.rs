use super::user::get_user_id;
use cores::request::JsonRespnse;

#[tauri::command]
pub async fn environment_trash_query_id(
    environment_uuid: String,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::environment_trash::query_by_uuid(&environment_uuid).await?)
}

#[tauri::command]
pub async fn environment_trash_query(
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::environment_trash::query(page_num, page_size).await?)
}

#[tauri::command]
pub async fn environment_trash_recover(
    environment_uuid: String,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::environment_trash::recover(&environment_uuid).await?)
}

#[tauri::command]
pub async fn environment_trash_recovers(
    environment_uuids: Vec<String>,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::environment_trash::recovers(environment_uuids).await?)
}

#[tauri::command]
pub async fn environment_trash_recover_all() -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::environment_trash::recover_all().await?)
}

#[tauri::command]
pub async fn environment_trash_delete_batch(
    environment_uuids: Vec<String>,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::environment_trash::delete_batch(environment_uuids).await?)
}

#[tauri::command]
pub async fn environment_trash_clean() -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::environment_trash::clean().await?)
}
