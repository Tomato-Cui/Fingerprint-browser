use lp_cores::requests::JsonRespnse;
use lp_models::environment_fingerprint::EnvironmentFingerprint;

#[tauri::command]
pub async fn environment_fingerprint_query_id(id: u32) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment_fingerprint::query_by_id(id).await?)
}

#[tauri::command]
pub async fn environment_fingerprint_query(
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment_fingerprint::query(page_num, page_size).await?)
}

#[tauri::command]
pub async fn environment_fingerprint_create(
    payload: EnvironmentFingerprint,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment_fingerprint::create(payload).await?)
}

#[tauri::command]
pub async fn environment_fingerprint_modify(
    payload: EnvironmentFingerprint,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment_fingerprint::modify(payload).await?)
}

#[tauri::command]
pub async fn environment_fingerprint_modify_ua(
    id: i32,
    ua: &str,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment_fingerprint::modify_ua(id, ua).await?)
}

#[tauri::command]
pub async fn environment_fingerprint_modify_by_colname(
    id: i32,
    col_name: &str,
    col_value: &str,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(
        lp_services_remote::requests::environment_fingerprint::modify_by_colname(
            id, col_name, col_value,
        )
        .await?,
    )
}

#[tauri::command]
pub async fn environment_fingerprint_delete(id: u32) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment_fingerprint::delete(id).await?)
}
