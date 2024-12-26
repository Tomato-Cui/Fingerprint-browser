use models::team::Team;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_id(id: u32) -> Result<Team, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let team = Team::query_team_by_id(pool, id).await?;

    Ok(team)
}

pub async fn query(user_uuid: &str, page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, team) =
        Team::query_teams_by_user_uuid(pool, user_uuid, page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": team,
    }))
}

pub async fn create(user_uuid: &str, payload: &Team) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Team::insert(pool, user_uuid, payload, None).await?;

    Ok(ok)
}

pub async fn modify(id: u32, payload: &Team) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Team::update(pool, id, payload).await?;

    Ok(ok)
}

pub async fn delete(id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Team::delete(pool, id).await?;

    Ok(ok)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_by_id() {
        let result = query_by_id(1).await;
        assert!(result.is_ok());
        let team = result.unwrap();
        assert_eq!(team.id, 1);
    }

    #[tokio::test]
    async fn test_query() {
        let result = query("user-uuid", 1, 10).await;
        assert!(result.is_ok());
        let value: Value = result.unwrap();
        assert!(value["total"].is_number());
        assert!(value["data"].is_array());
    }

    #[tokio::test]
    async fn test_create() {
        let team = Team {
            ..Default::default()
        };
        let result = create("user-uuid", &team).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_modify() {
        let team = Team {
            ..Default::default()
        };
        let result = modify(1, &team).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_delete() {
        let result = delete(1).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}
