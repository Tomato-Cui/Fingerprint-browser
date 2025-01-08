use super::user::get_user_id;
use cores::request::JsonRespnse;

#[tauri::command]
pub async fn team_is_leader(team_id: u32) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::team::is_leader(team_id).await?)
}

#[tauri::command]
pub async fn team_query_name(team_name: String) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::team::query_search_by_name(&team_name).await?)
}

#[tauri::command]
pub async fn team_query_id(id: u32) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::team::query_by_id(id).await?)
}

#[tauri::command]
pub async fn team_query(page_num: u32, page_size: u32) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::team::query(page_num, page_size).await?)
}

#[tauri::command]
pub async fn query_current_team_info() -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::team::query_current_team().await?)
}

#[tauri::command]
pub async fn query_team_all_user(
    team_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::team::query_team_all_user(team_id, page_num, page_size).await?)
}

#[tauri::command]
pub async fn query_team_all_blocked_user(
    team_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(
        services_remote::requests::team::query_team_all_blocked_user(team_id, page_num, page_size)
            .await?,
    )
}

#[tauri::command]
pub async fn query_team_group_all_user(
    team_id: u32,
    team_group_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::team::query_team_group_all_user(
        team_id,
        team_group_id,
        page_num,
        page_size,
    )
    .await?)
}

#[tauri::command]
pub async fn team_create(name: String, description: String) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::team::create(&name, &description).await?)
}

#[tauri::command]
pub async fn un_blocked(
    current_user_uuid: String,
    team_id: u32,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::team::unblocked(&current_user_uuid, team_id).await?)
}

#[tauri::command]
pub async fn blocked(current_user_uuid: String, team_id: u32) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::team::blocked(&current_user_uuid, team_id).await?)
}

#[tauri::command]
pub async fn team_modify(
    id: u32,
    name: String,
    description: String,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::team::modify(id, &name, &description).await?)
}

#[tauri::command]
pub async fn switch_team(id: u32) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::team::switch_team(id).await?)
}

#[tauri::command]
pub async fn remove_current_user(
    team_id: u32,
    current_user_uuid: String,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::team::remove_current_user(team_id, &current_user_uuid).await?)
}

#[tauri::command]
pub async fn team_modify_team_user_info(
    team_id: u32,
    description: Option<String>,
    team_group_id: u32,
    current_user_uuid: String,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::team::modify_team_user_info(
        team_id,
        description,
        team_group_id,
        &current_user_uuid,
    )
    .await?)
}

#[tauri::command]
pub async fn team_delete(id: u32) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::team::delete(id).await?)
}
