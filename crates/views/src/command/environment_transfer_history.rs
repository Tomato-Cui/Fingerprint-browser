use super::user::get_user_id;
use cores::request::JsonRespnse;

#[tauri::command]
pub async fn environment_transfer_history_query_id(
    environment_uuid: String,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(
        services_remote::requests::environment_transfer_history::query_by_uuid(&environment_uuid)
            .await?,
    )
}

#[tauri::command]
pub async fn environment_transfer_history_query(
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(services_remote::requests::environment_transfer_history::query(page_num, page_size).await?)
}

#[tauri::command]
pub async fn environment_transfer_history_batch_create(
    current_user_email: String,
    environment_uuids: Vec<String>,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(
        services_remote::requests::environment_transfer_history::batch_create(
            environment_uuids,
            &current_user_email,
        )
        .await?,
    )
}

#[tauri::command]
pub async fn environment_transfer_history_delete(
    environment_uuid: String,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(
        services_remote::requests::environment_transfer_history::delete_by_uuid(&environment_uuid)
            .await?,
    )
}
