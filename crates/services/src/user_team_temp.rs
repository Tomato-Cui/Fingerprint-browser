use models::user_team_temp::UserTeamTemp;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_user_apply(
    user_uuid: &str,
    page_num: u32,
    page_size: u32,
) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, user_team_temps) =
        UserTeamTemp::query_user_apply(pool, user_uuid, page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": user_team_temps,
    }))
}

pub async fn query_team_apply(
    team_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, user_team_temps) =
        UserTeamTemp::query_team_apply(pool, team_id, page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": user_team_temps,
    }))
}

pub async fn user_send(
    user_uuid: &str,
    team_id: u32,
    description: &str,
) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let user_team_temp = UserTeamTemp {
        user_uuid: user_uuid.to_string(),
        team_id,
        description: Some(description.to_string()),
        allow_1: Some(1),
        allow_2: Some(0),
        ..Default::default()
    };

    let ok = UserTeamTemp::insert_user_team_temp(pool, &user_team_temp).await?;

    Ok(ok)
}

pub async fn team_send(
    user_uuid: &str,
    team_id: u32,
    description: &str,
) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let user_team_temp = UserTeamTemp {
        user_uuid: user_uuid.to_string(),
        team_id,
        description: Some(description.to_string()),
        allow_1: Some(0),
        allow_2: Some(1),
        ..Default::default()
    };

    let ok = UserTeamTemp::insert_user_team_temp(pool, &user_team_temp).await?;

    Ok(ok)
}

pub async fn allow(id: u32, user_uuid: &str, team_id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let user_team_temp = UserTeamTemp {
        user_uuid: user_uuid.to_string(),
        team_id,
        allow_1: Some(1),
        allow_2: Some(1),
        ..Default::default()
    };

    let ok = UserTeamTemp::agree_join(pool, id, &user_team_temp).await?;
    Ok(ok)
}

pub async fn delete(id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = UserTeamTemp::delete_user_team_temp(pool, id).await?;

    Ok(ok)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_user_apply() {
        crate::setup().await;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5";
        let page_num = 1;
        let page_size = 10;

        let result = query_user_apply(user_uuid, page_num, page_size).await;
        let value = result.unwrap();
        println!("{:?}", value)
    }

    #[tokio::test]
    async fn test_query_team_apply() {
        crate::setup().await;
        let team_id = 4;
        let page_num = 1;
        let page_size = 10;

        let result = query_team_apply(team_id, page_num, page_size).await;
        let value = result.unwrap();
        println!("{:?}", value)
    }

    #[tokio::test]
    async fn test_user_send() {
        crate::setup().await;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5";
        let team_id = 4;
        let description = "test_description";

        let result = user_send(user_uuid, team_id, description).await;
        println!("{:?}", result)
    }

    #[tokio::test]
    async fn test_team_send() {
        crate::setup().await;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5";
        let team_id = 4;
        let description = "test_description";

        let result = team_send(user_uuid, team_id, description).await;
        println!("{:?}", result)
    }

    #[tokio::test]
    async fn test_allow() {
        crate::setup().await;
        let id = 10;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5";
        let team_id = 4;

        let result = allow(id, user_uuid, team_id).await;
        println!("{:?}", result)
    }

    #[tokio::test]
    async fn test_delete() {
        crate::setup().await;
        let id = 10;

        let result = delete(id).await;
        println!("{:?}", result)
    }
}
