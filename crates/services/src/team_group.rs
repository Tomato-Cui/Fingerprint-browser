use models::team_group::TeamGroup;

use crate::error::ServiceError;

pub async fn query_by_id(id: u32) -> Result<TeamGroup, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let group = TeamGroup::query_team_group_by_id(pool, id).await?;

    Ok(group)
}

pub async fn query_by_team_id(id: u32) -> Result<serde_json::Value, ServiceError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_by_id() {
        let result = query_by_id(1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_query_by_team_id() {
        crate::setup().await;
        let result = query_by_team_id(1).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_create() {
        let payload = TeamGroup {
            ..Default::default()
        };
        let result = create(&payload).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_modify() {
        let payload = TeamGroup {
            ..Default::default()
        };
        let result = modify(&payload).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete() {
        let result = delete(1).await;
        assert!(result.is_ok());
    }
}
