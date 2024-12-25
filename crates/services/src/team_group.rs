use models::team_group::TeamGroup;

use crate::error::ServiceError;

pub async fn query_by_id(id: u32) -> Result<TeamGroup, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let group = TeamGroup::query_team_group_by_id(pool, id).await?;

    Ok(group)
}

pub async fn query_by_team_id(id: u32) -> Result<Vec<TeamGroup>, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let groups = TeamGroup::query_team_group_by_team_id(pool, id).await?;

    Ok(groups)
}

pub async fn create(payload: &TeamGroup) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = TeamGroup::insert_team_group(pool, &payload).await?;

    Ok(ok)
}

pub async fn modify(payload: &TeamGroup) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = TeamGroup::update_team_group(pool, payload).await?;

    Ok(ok)
}

pub async fn delete(id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = TeamGroup::delete_team_group(pool, id).await?;

    Ok(ok)
}
