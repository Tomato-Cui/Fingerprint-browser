use lp_cores::requests::JsonRespnse as JsonResponse;

#[tauri::command]
pub async fn environment_tag_query_id(id: u32) -> Result<JsonResponse, tauri::Error> {
    Ok(lp_services_remote::requests::environment_tag::query_by_id(id).await?)
}

#[tauri::command]
pub async fn environment_tag_query(
    page_num: u32,
    page_size: u32,
) -> Result<JsonResponse, tauri::Error> {
    Ok(lp_services_remote::requests::environment_tag::query(page_num, page_size).await?)
}

#[tauri::command]
pub async fn environment_tag_create(
    name: String,
    description: String,
) -> Result<JsonResponse, tauri::Error> {
    Ok(lp_services_remote::requests::environment_tag::create(&name, Some(description)).await?)
}

#[tauri::command]
pub async fn environment_tag_modify(
    id: u32,
    name: String,
    description: String,
) -> Result<JsonResponse, tauri::Error> {
    Ok(lp_services_remote::requests::environment_tag::modify(id, &name, Some(description)).await?)
}

#[tauri::command]
pub async fn environment_tag_delete(id: u32) -> Result<JsonResponse, tauri::Error> {
    Ok(lp_services_remote::requests::environment_tag::delete(id).await?)
}
