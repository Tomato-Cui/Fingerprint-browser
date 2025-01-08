use super::user::get_user_id;
use cores::request::JsonRespnse;

#[tauri::command]
pub async fn environment_group_query_id(id: u32) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::environment_group::query_by_id(id).await?)
}

#[tauri::command]
pub async fn environment_group_query(
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::environment_group::query(page_num, page_size).await?)
}

#[tauri::command]
pub async fn environment_group_create(
    name: String,
    description: String,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::environment_group::create(&name, Some(description)).await?)
}

#[tauri::command]
pub async fn environment_group_modify(
    id: u32,
    name: String,
    description: String,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::environment_group::modify(id, &name, Some(description)).await?)
}

#[tauri::command]
pub async fn environment_group_delete(id: u32) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::environment_group::delete(id).await?)
}
