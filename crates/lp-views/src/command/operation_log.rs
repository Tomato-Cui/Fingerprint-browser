use lp_cores::requests::JsonRespnse;

#[tauri::command]
pub async fn opeartion_query(page_num: u32, page_size: u32) -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::operation_log::query(page_num, page_size).await?)
}

#[tauri::command]
pub async fn opeartion_query_by_team(
    team_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    Ok(
        lp_services_remote::requests::operation_log::query_by_team(team_id, page_num, page_size)
            .await?,
    )
}
