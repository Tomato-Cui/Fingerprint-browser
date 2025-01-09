use cores::request::JsonRespnse;

#[tauri::command]
pub async fn environment_proxy_group_query_id(id: u32) -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::environment_proxy_group::query_by_id(id).await?)
}

#[tauri::command]
pub async fn environment_proxy_group_query(
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::environment_proxy_group::query(page_num, page_size).await?)
}

#[tauri::command]
pub async fn environment_proxy_group_create(
    payload: models::environment_proxy_group::ProxyGroup,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::environment_proxy_group::create(payload).await?)
}

#[tauri::command]
pub async fn environment_proxy_group_modify(
    payload: models::environment_proxy_group::ProxyGroup,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::environment_proxy_group::modify(payload).await?)
}

#[tauri::command]
pub async fn environment_proxy_group_delete(id: u32) -> Result<JsonRespnse, tauri::Error> {
    Ok(services_remote::requests::environment_proxy_group::delete(id).await?)
}
