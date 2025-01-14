use lp_cores::requests::JsonRespnse;

#[tauri::command]
pub async fn environment_query_id(environment_uuid: String) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment::query_by_uuid(&environment_uuid).await?)
}

#[tauri::command]
pub async fn environment_query(page_num: u32, page_size: u32) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment::query(page_num, page_size).await?)
}

#[tauri::command]
pub async fn environment_query_by_group(
    id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment::query_by_group(id, page_num, page_size).await?)
}

#[tauri::command]
pub async fn environment_query_by_team(
    id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment::query_by_team(id, page_num, page_size).await?)
}

#[tauri::command]
pub async fn environment_query_by_extension(
    extension_uuid: String,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(
        lp_services_remote::requests::environment::query_by_extension(
            &extension_uuid,
            page_num,
            page_size,
        )
        .await?,
    )
}

#[tauri::command]
pub async fn environment_detail_create(
    payload: lp_models::environment::EnvironmentInfo,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment::detail_create(payload).await?)
}

#[tauri::command]
pub async fn environment_simple_create(
    browser_type: &str,
    os_type: &str,
    numbers: u32,
    group_id: Option<u32>,
    use_encrypt: bool,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment::simple_create(
        browser_type,
        os_type,
        numbers,
        group_id,
        use_encrypt,
    )
    .await?)
}

#[tauri::command]
pub async fn environment_batch_create(
    environment_names: Vec<String>,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment::batch_create(environment_names).await?)
}

#[tauri::command]
pub async fn environment_modify_info(
    payload: lp_models::environment::EnvironmentInfo,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment::modify_info(payload).await?)
}

#[tauri::command]
pub async fn environment_modify_proxy(
    environment_uuid: String,
    payload: lp_models::environment_proxies::Proxy,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment::modify_proxy(&environment_uuid, payload).await?)
}

#[tauri::command]
pub async fn environment_modify_basic_info(
    payload: lp_models::environment::Environment,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment::modify_basic_info(payload).await?)
}

#[tauri::command]
pub async fn environment_move_to_group(
    environment_uuid: String,
    group_id: u32,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(
        lp_services_remote::requests::environment::move_to_group(&environment_uuid, group_id)
            .await?,
    )
}

#[tauri::command]
pub async fn environment_batch_move_to_group(
    environment_uuids: Vec<String>,
    group_id: u32,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(
        lp_services_remote::requests::environment::batch_move_to_group(environment_uuids, group_id)
            .await?,
    )
}

#[tauri::command]
pub async fn environment_delete(environment_uuid: String) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment::delete(&environment_uuid).await?)
}

#[tauri::command]
pub async fn environment_batch_delete(
    environment_uuids: Vec<String>,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment::batch_delete(environment_uuids).await?)
}
