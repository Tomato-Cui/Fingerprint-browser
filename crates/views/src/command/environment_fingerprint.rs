use cores::request::JsonRespnse;
use models::environment_fingerprint::EnvironmentFingerprint;

use super::user::get_user_id;

#[tauri::command]
pub async fn environment_fingerprint_query_id(id: u32) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::environment_fingerprint::query_by_id(id).await?)
}

#[tauri::command]
pub async fn environment_fingerprint_query(
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::environment_fingerprint::query(page_num, page_size).await?)
}

#[tauri::command]
pub async fn environment_fingerprint_create(
    payload: EnvironmentFingerprint,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::environment_fingerprint::create(payload).await?)
}

#[tauri::command]
pub async fn environment_fingerprint_modify(
    id: u32,
    mut payload: EnvironmentFingerprint,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    payload.id = Some(id as i32);
    Ok(services_remote::requests::environment_fingerprint::modify(payload).await?)
}

#[tauri::command]
pub async fn environment_fingerprint_delete(id: u32) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::environment_fingerprint::delete(id).await?)
}
