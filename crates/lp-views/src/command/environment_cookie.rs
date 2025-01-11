use lp_cores::requests::JsonRespnse;
use lp_services_remote::requests::environment_cookie;

#[tauri::command]
pub async fn environment_cookie_query_environment_uuid(
    environment_uuid: String,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(environment_cookie::query_by_uuid(&environment_uuid).await?)
}

#[tauri::command]
pub async fn environment_cookie_create(
    environment_uuid: String,
    cookie_str: String,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(environment_cookie::create(&environment_uuid, &cookie_str).await?)
}

#[tauri::command]
pub async fn environment_cookie_modify(
    environment_uuid: String,
    cookie_str: String,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(environment_cookie::modify(&environment_uuid, &cookie_str).await?)
}

#[tauri::command]
pub async fn environment_cookie_delete(
    environmnet_uuid: &str,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(environment_cookie::delete(&environmnet_uuid).await?)
}
