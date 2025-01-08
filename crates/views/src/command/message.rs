use super::user::get_user_id;
use cores::request::JsonRespnse;

#[tauri::command]
pub async fn user_receive_query(
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::message::user_receive_query(page_num, page_size).await?)
}

#[tauri::command]
pub async fn team_receive_query(
    team_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(
        services_remote::requests::message::team_receive_query(team_id, page_num, page_size)
            .await?,
    )
}

#[tauri::command]
pub async fn team_send(
    team_id: u32,
    current_user_email: String,
    description: String,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(
        services_remote::requests::message::team_send(&current_user_email, team_id, &description)
            .await?,
    )
}

#[tauri::command]
pub async fn user_send(
    team_name: String,
    description: String,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::message::user_send(&team_name, &description).await?)
}

#[tauri::command]
pub async fn reject(id: u32) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::message::reject(id).await?)
}

#[tauri::command]
pub async fn team_allow(
    id: u32,
    user_uuid: String,
    team_id: u32,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::message::team_allow(id, &user_uuid, team_id).await?)
}

#[tauri::command]
pub async fn user_allow(id: u32, team_id: u32) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::message::user_allow(id, team_id).await?)
}
