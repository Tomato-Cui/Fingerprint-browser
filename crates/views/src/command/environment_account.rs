use cores::request::JsonRespnse;
use models::environment_account::EnvironmentAccount;
use services_remote::requests::environment_account;

#[tauri::command]
pub async fn environment_account_query_id(id: u32) -> Result<JsonRespnse, tauri::Error> {
    Ok(environment_account::query_by_id(id).await?)
}

#[tauri::command]
pub async fn environment_account_query(
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(environment_account::query(page_num, page_size).await?)
}

#[tauri::command]
pub async fn environment_account_query_current(
    environmnet_uuid: &str,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(environment_account::query_current_by_current_environment(
        environmnet_uuid,
        page_num,
        page_size,
    )
    .await?)
}

#[tauri::command]
pub async fn environment_account_create(
    payload: EnvironmentAccount,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(environment_account::create(payload).await?)
}

#[tauri::command]
pub async fn environment_account_modify(
    payload: EnvironmentAccount,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(environment_account::modify(payload).await?)
}

#[tauri::command]
pub async fn environment_account_delete(id: u32) -> Result<JsonRespnse, tauri::Error> {
    Ok(environment_account::delete(id).await?)
}

#[tauri::command]
pub async fn environment_account_batch_delete(ids: Vec<u32>) -> Result<JsonRespnse, tauri::Error> {
    Ok(environment_account::batch_delete(ids).await?)
}
