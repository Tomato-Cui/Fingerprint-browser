use lp_models::team_group_permission;

use crate::error::ServiceError;

pub async fn can_check_permission(resource_name: &str) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let ok = team_group_permission::TeamGroupPermission::can_check_permission(pool, resource_name)
        .await?;

    Ok(ok)
}

pub async fn check_permission(user_uuid: &str, resource_name: &str) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let ok = team_group_permission::TeamGroupPermission::check_permission(
        pool,
        user_uuid,
        resource_name,
    )
    .await?;

    Ok(ok)
}

pub async fn grant_permission(
    user_uuid: &str,
    team_id: u32,
    resource_name: &str,
) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let ok = team_group_permission::TeamGroupPermission::grant_permission(
        pool,
        user_uuid,
        team_id,
        resource_name,
    )
    .await?;

    Ok(ok)
}

#[tokio::test]
async fn test_can_check_permission() {
    crate::setup().await;
    let res = can_check_permission("/api/v1/environments+GET").await;

    println!("{:?}", res);
}

#[tokio::test]
async fn test_check_permission() {
    crate::setup().await;
    let res = check_permission(
        "0a3b8745-cd95-48c3-b4cb-58aae0ccd87b",
        "/api/v1/environments+GET",
    )
    .await;

    println!("{:?}", res);
}
