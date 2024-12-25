use crate::response::AppResponse;
use models::team_group::TeamGroup;

use super::user::get_user_id;

#[tauri::command]
pub async fn team_group_query_id(id: u32) -> Result<AppResponse<TeamGroup>, tauri::Error> {
    let _ = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    Ok(match services::team_group::query_by_id(id).await {
        Ok(data) => AppResponse::<TeamGroup>::success(success_msg, Some(data)),
        Err(r) => AppResponse::<TeamGroup>::fail(warn_msg(r.to_string())),
    })
}

#[tauri::command]
pub async fn team_group_query_all(
    team_id: u32,
) -> Result<AppResponse<Vec<TeamGroup>>, tauri::Error> {
    let _ = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    Ok(
        match services::team_group::query_by_team_id(team_id).await {
            Ok(data) => AppResponse::<Vec<TeamGroup>>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Vec<TeamGroup>>::fail(warn_msg(r.to_string())),
        },
    )
}
