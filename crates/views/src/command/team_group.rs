use super::user::get_user_id;
use cores::request::JsonRespnse;

#[tauri::command]
pub async fn team_group_query_id(id: u32) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::team_group::query_by_id(id).await?)
}

#[tauri::command]
pub async fn team_group_query_all(team_id: u32) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::team_group::query_all(team_id).await?)
}