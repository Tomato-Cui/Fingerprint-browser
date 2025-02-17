// 导入JSON响应类型
use lp_cores::requests::JsonRespnse;

// 根据ID查询代理分组
#[tauri::command]
pub async fn environment_proxy_group_query_id(id: u32) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment_proxy_group::query_by_id(id).await?)
}

// 分页查询代理分组列表
#[tauri::command]
pub async fn environment_proxy_group_query(
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment_proxy_group::query(page_num, page_size).await?)
}

// 创建新的代理分组
#[tauri::command]
pub async fn environment_proxy_group_create(
    payload: lp_models::environment_proxy_group::ProxyGroup,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment_proxy_group::create(payload).await?)
}

// 修改代理分组信息
#[tauri::command]
pub async fn environment_proxy_group_modify(
    payload: lp_models::environment_proxy_group::ProxyGroup,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment_proxy_group::modify(payload).await?)
}

// 删除代理分组
#[tauri::command]
pub async fn environment_proxy_group_delete(id: u32) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::environment_proxy_group::delete(id).await?)
}
