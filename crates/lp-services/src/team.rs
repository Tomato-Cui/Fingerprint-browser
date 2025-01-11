use lp_models::team::Team;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn is_leader(user_uuid: &str, team_id: u32) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;
    let team = Team::is_leader(pool, user_uuid, team_id).await?;

    Ok(team)
}

pub async fn query_by_search_name(team_name: &str) -> Result<Vec<Team>, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;
    let team = Team::query_team_search_name(pool, team_name).await?;

    Ok(team)
}

pub async fn query_by_id(id: u32) -> Result<Team, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;
    let team = Team::query_team_by_id(pool, id).await?;

    Ok(team)
}

pub async fn query(user_uuid: &str, page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let (total, team) =
        Team::query_teams_by_user_uuid(pool, user_uuid, page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": team,
    }))
}

pub async fn query_current_team_info(user_uuid: &str) -> Result<Team, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let current_team_id =
        lp_models::user_use_team::UserUseTeam::query_current_team_id_by_user_uuid(pool, user_uuid)
            .await?;

    let team = Team::query_team_by_id(pool, current_team_id).await?;

    Ok(team)
}

pub async fn query_team_all_user(
    user_uuid: &str,
    team_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<Value, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

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
    let pool = lp_states::database::get_database_pool()?;

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
    let pool = lp_states::database::get_database_pool()?;

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
    let pool = lp_states::database::get_database_pool()?;

    let (total, team) =
        Team::query_teams_by_user_uuid(pool, user_uuid, page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": team,
    }))
}

pub async fn create(user_uuid: &str, payload: &Team) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let ok = Team::insert(pool, user_uuid, payload, None).await?;

    Ok(ok)
}

pub async fn blocked(
    user_uuid: &str,
    current_user_uuid: &str,
    team_id: u32,
) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let ok = Team::blocked_action(pool, user_uuid, team_id, current_user_uuid, true).await?;

    Ok(ok)
}

pub async fn un_blocked(
    user_uuid: &str,
    current_user_uuid: &str,
    team_id: u32,
) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;
    let ok = Team::blocked_action(pool, user_uuid, team_id, &current_user_uuid, false).await?;

    Ok(ok)
}

pub async fn remove_user(
    user_uuid: &str,
    team_id: u32,
    current_user_uuid: &str,
) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let ok = Team::remove_user(pool, user_uuid, team_id, current_user_uuid).await?;

    Ok(ok)
}

pub async fn switch_team(user_uuid: &str, team_id: u32) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let ok = Team::switch_team(pool, user_uuid, team_id).await?;

    Ok(ok)
}

pub async fn modify(id: u32, payload: &Team) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let ok = Team::update(pool, id, payload).await?;

    Ok(ok)
}

pub async fn update_team_user_info(
    user_uuid: &str,
    team_id: u32,
    description: Option<String>,
    team_group_id: u32,
    current_user_uuid: &str,
) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let ok = Team::update_team_user_info(
        pool,
        user_uuid,
        team_id,
        description,
        team_group_id,
        current_user_uuid,
    )
    .await?;

    Ok(ok)
}

pub async fn delete(id: u32) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

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
    async fn test_blocked() {
        crate::setup().await;
        let result = blocked(
            "6c16972f-fcdf-46ad-9c62-e799377eb108",
            "6c16972f-fcdf-46ad--e799377eb10",
            1,
        )
        .await;

        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_un_blocked() {
        crate::setup().await;
        let result = un_blocked(
            "6c16972f-fcdf-46ad-9c62-e799377eb108",
            "6c16972f-fcdf-46ad--e799377eb10",
            1,
        )
        .await;

        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_remove_user() {
        crate::setup().await;
        let user_uuid = "";
        let team_id = 1;
        let crrent_user_uuid = "";
        let result = remove_user(user_uuid, team_id, crrent_user_uuid).await;

        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_query_team_all_user() {
        crate::setup().await;
        let result = query_team_all_user("d3129a09-5473-4b1e-915b-bba0af78d752", 1, 1, 10).await;

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
            query_team_group_all_user("d3129a09-5473-4b1e-915b-bba0af78d752", 1, 1, 1, 10).await;

        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_query_by_search_name() {
        crate::setup().await;
        let result = query_by_search_name("liushuinew").await;

        println!("{:?}", result);
    }
}
