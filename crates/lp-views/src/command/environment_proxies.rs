use lp_cores::requests::JsonRespnse;

#[tauri::command]
pub async fn environment_proxies_query_id(id: u32) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment_proxies::query_by_id(id).await?)
}

#[tauri::command]
pub async fn environment_proxies_query(
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment_proxies::query(page_num, page_size).await?)
}

#[tauri::command]
pub async fn environment_proxies_query_by_group(
    proxy_group_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(
        lp_services_remote::requests::environment_proxies::query_by_group(
            proxy_group_id,
            page_num,
            page_size,
        )
        .await?,
    )
}

#[tauri::command]
pub async fn environment_proxies_create(
    payload: lp_models::environment_proxies::Proxy,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment_proxies::create(payload).await?)
}

#[tauri::command]
pub async fn environment_proxies_modify(
    payload: lp_models::environment_proxies::Proxy,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment_proxies::modify(payload).await?)
}

#[tauri::command]
pub async fn environment_proxies_delete(id: u32) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment_proxies::delete(id).await?)
}

#[tauri::command]
pub async fn environment_proxies_batch_delete(ids: Vec<u32>) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment_proxies::batch_delete(ids).await?)
}
