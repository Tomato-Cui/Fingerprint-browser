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

pub async fn query_team_all_user(
    user_uuid: &str,
    team_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, team) =
        Team::query_team_all_user_by_team_id(pool, user_uuid, team_id, false, page_num, page_size)
            .await?;

    Ok(json!({
        "total": total,
        "data": team,
    }))
}

pub async fn query_team_all_blocked_user(
    user_uuid: &str,
    team_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, team) =
        Team::query_team_all_user_by_team_id(pool, user_uuid, team_id, true, page_num, page_size)
            .await?;

    Ok(json!({
        "total": total,
        "data": team,
    }))
}

pub async fn query_team_group_all_user(
    user_uuid: &str,
    team_id: u32,
    group_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, team) = Team::query_team_all_user_by_group_id(
        pool, user_uuid, team_id, group_id, page_num, page_size,
    )
    .await?;

    Ok(json!({
        "total": total,
        "data": team,
    }))
}

pub async fn query_current_user_with_team(
    user_uuid: &str,
    page_num: u32,
    page_size: u32,
) -> Result<Value, ServiceError> {
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
        crate::setup().await;
        let result = query_by_id(1).await;
        let team = result.unwrap();
        println!("{:?}", team);
    }

    #[tokio::test]
    async fn test_query() {
        crate::setup().await;
        let result = query("3cfb0bc6-7b48-498a-935a-90ce561e40a5", 1, 10).await;
        let value: Value = result.unwrap();
        println!("{:?}", value);
    }

    #[tokio::test]
    async fn test_create() {
        crate::setup().await;
        let team = Team {
            name: "sdfhfj".to_string(),
            ..Default::default()
        };
        let result = create("3cfb0bc6-7b48-498a-935a-90ce561e40a5", &team).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_modify() {
        crate::setup().await;
        let team = Team {
            name: "sdfhjdfhs".to_string(),
            ..Default::default()
        };
        let result = modify(1, &team).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_delete() {
        crate::setup().await;
        let result = delete(1).await;

        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_query_team_all_user() {
        crate::setup().await;
        let result = query_team_all_user("3cfb0bc6-7b48-498a-935a-90ce561e40a5", 4, 1, 10).await;

        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_query_team_all_blocked_user() {
        crate::setup().await;
        let result =
            query_team_all_blocked_user("3cfb0bc6-7b48-498a-935a-90ce561e40a5", 4, 1, 10).await;

        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_query_team_group_all_user() {
        crate::setup().await;
        let result =
            query_team_group_all_user("3cfb0bc6-7b48-498a-935a-90ce561e40a5", 4, 14, 1, 10).await;

        println!("{:?}", result);
    }
}
