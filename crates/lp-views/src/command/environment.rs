use lp_cores::requests::JsonRespnse;

// 根据环境UUID查询环境信息
#[tauri::command]
pub async fn environment_query_id(environment_uuid: String) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment::query_by_uuid(&environment_uuid).await?)
}

// 分页查询所有环境
#[tauri::command]
pub async fn environment_query(page_num: u32, page_size: u32) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment::query(page_num, page_size).await?)
}

// 根据分组ID分页查询环境
#[tauri::command]
pub async fn environment_query_by_group(
    id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment::query_by_group(id, page_num, page_size).await?)
}

// 根据团队ID分页查询环境
#[tauri::command]
pub async fn environment_query_by_team(
    id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment::query_by_team(id, page_num, page_size).await?)
}

// 根据扩展UUID分页查询环境
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

// 简单创建环境
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

// 高级创建环境
#[tauri::command]
pub async fn environment_advanced_create(
    numbers: u32,
    use_encrypt: bool,
    environment: lp_models::dto::environment_info::EnvironmentDetailWithAdvanceCreateRequest,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment::advanced_create(
        numbers,
        use_encrypt,
        environment,
    )
    .await?)
}

// 高级修改环境
#[tauri::command]
pub async fn environment_advanced_modify(
    environment_uuid: &str,
    payload: lp_models::dto::environment_info::EnvironmentDetailWithAdvanceCreateRequest,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(
        lp_services_remote::requests::environment::advanced_modify(environment_uuid, payload)
            .await?,
    )
}

// 修改环境默认URL
#[tauri::command]
pub async fn environment_modify_default_url(
    environment_uuid: &str,
    default_urls: &str,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(
        lp_services_remote::requests::environment::modify_default_url(
            environment_uuid,
            default_urls,
        )
        .await?,
    )
}

// 修改环境信息
#[tauri::command]
pub async fn environment_modify_info(
    payload: lp_models::environment::EnvironmentInfo,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment::modify_info(payload).await?)
}

// 修改环境代理
#[tauri::command]
pub async fn environment_modify_proxy(
    environment_uuid: String,
    payload: lp_models::environment_proxies::Proxy,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment::modify_proxy(&environment_uuid, payload).await?)
}

// 修改环境基本信息
#[tauri::command]
pub async fn environment_modify_basic_info(
    payload: lp_models::environment::Environment,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment::modify_basic_info(payload).await?)
}

// 将环境移动到指定分组
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

// 批量将环境移动到指定标签
#[tauri::command]
pub async fn environment_batch_move_to_tag(
    environment_uuids: Vec<String>,
    tag_id: u32,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(
        lp_services_remote::requests::environment::batch_move_to_tag(environment_uuids, tag_id)
            .await?,
    )
}

// 批量将环境移动到指定分组
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

// 删除环境
#[tauri::command]
pub async fn environment_delete(environment_uuid: String) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment::delete(&environment_uuid).await?)
}

// 批量删除环境
#[tauri::command]
pub async fn environment_batch_delete(
    environment_uuids: Vec<String>,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment::batch_delete(environment_uuids).await?)
}
