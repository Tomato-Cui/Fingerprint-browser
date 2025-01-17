use crate::error::ServiceError;
use lp_models::environment::{Environment};
use serde_json::{json, Value};

pub async fn query_by_uuid(uuid: &str) -> Result<Environment, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;
    let environment = lp_models::environment::Environment::query_by_uuid(pool, uuid).await?;

    Ok(environment)
}

pub async fn query_environment_details(user_uuid: &str, uuid: &str) -> Result<Value, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;
    let environment =
        lp_models::environment::Environment::query_environment_details(pool, uuid, user_uuid).await?;

    Ok(environment)
}

pub async fn query(user_uuid: &str, page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;
    let current_team_id = crate::team::query_current_team_info(user_uuid).await?.id as u32;
    let is_leader = crate::team::is_leader(user_uuid, current_team_id).await?;

    let (total, environment) = if is_leader {
        lp_models::environment::Environment::query_environments_by_user_uuid(
            pool, user_uuid, page_num, page_size,
        )
        .await?
    } else {
        lp_models::environment::Environment::query_environments_by_team_id_and_user_uuid(
            pool,
            user_uuid,
            current_team_id,
            page_num,
            page_size,
        )
        .await?
    };

    lp_models::environment::Environment::query_environments_by_user_uuid(
        pool, user_uuid, page_num, page_size,
    )
    .await?;

    Ok(json!({
        "total": total,
        "data": environment,
    }))
}

pub async fn query_by_team_id(
    user_uuid: &str,
    team_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<Value, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;
    let (total, environment) =
        lp_models::environment::Environment::query_environments_by_team_id_and_user_uuid(
            pool, user_uuid, team_id, page_num, page_size,
        )
        .await?;

    Ok(json!({
        "total": total,
        "data": environment,
    }))
}

pub async fn query_by_group_id(
    group_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<Value, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;
    let (total, environment) = lp_models::environment::Environment::query_environments_by_group_id(
        pool, group_id, page_num, page_size,
    )
    .await?;

    Ok(json!({
        "total": total,
        "data": environment,
    }))
}



pub async fn move_to_group(env_uuid: &str, group_id: u32) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;
    let ok = lp_models::environment::Environment::move_environment_to_group(pool, env_uuid, group_id)
        .await?;

    Ok(ok)
}

pub async fn batch_move_to_group(
    env_uuids: Vec<String>,
    group_id: u32,
) -> Result<Vec<Value>, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let mut response = vec![];
    for env_id in env_uuids {
        let ok =
            lp_models::environment::Environment::move_environment_to_group(pool, &env_id, group_id)
                .await?;
        response.push(json!({
            "environment_id": env_id,
            "success": ok,
        }));
    }
    Ok(response)
}

pub async fn modify_basic_info(
    uuid: &str,
    name: &str,
    description: Option<String>,
) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let ok =
        lp_models::environment::Environment::modify_basic_info(pool, uuid, name, description).await?;

    Ok(ok)
}

pub async fn modify_info(
    user_uuid: &str,
    environment_uuid: &str,
    environment: lp_models::environment::EnvironmentInfo,
) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let ok = lp_models::environment::Environment::modify_and_other_info(
        pool,
        user_uuid,
        environment_uuid,
        &environment,
    )
    .await?;

    Ok(ok)
}

pub async fn delete(user_uuid: &str, env_uuid: &str) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let ok = lp_models::environment::Environment::delete_and_move_to_trash(pool, env_uuid, user_uuid)
        .await?;

    Ok(ok)
}

pub async fn batch_delete(
    user_uuid: &str,
    env_ids: Vec<String>,
) -> Result<Vec<bool>, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let mut response = vec![];
    for env_id in env_ids {
        let ok =
            lp_models::environment::Environment::delete_and_move_to_trash(pool, &env_id, user_uuid)
                .await?;
        response.push(ok);
    }
    Ok(response)
}

#[cfg(test)]
mod tests {
    use lp_models::environment_fingerprint::EnvironmentFingerprint;

    use super::*;

    #[tokio::test]
    async fn test_query_by_uuid() {
        crate::setup().await;
        let uuid = "ac19b5cc-5a84-490d-b913-452ee71c52c7";
        let result = query_by_uuid(uuid).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_query_env_info() {
        crate::setup().await;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5";
        let uuid = "b13d4d98-f3da-4479-97b9-6de4975aa97b";
        let result = query_environment_details(&user_uuid, uuid).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_query() {
        crate::setup().await;
        let user_uuid = "ac19b5cc-5a84-490d-b913-452ee71c52c7";
        let page_num = 1;
        let page_size = 10;
        let result = query(user_uuid, page_num, page_size).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_query_by_team_id() {
        crate::setup().await;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5";
        let team_id = 3;
        let page_num = 1;
        let page_size = 10;
        let result = query_by_team_id(user_uuid, team_id, page_num, page_size).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_query_by_group_id() {
        crate::setup().await;
        let group_id = 1;
        let page_num = 1;
        let page_size = 10;
        let result = query_by_group_id(group_id, page_num, page_size).await;
        println!("{:?}", result);
    }



    #[tokio::test]
    async fn test_move_to_group() {
        crate::setup().await;
        let env_uuid = "2b7e8675-cebe-45d1-81c2-5fb0aa1d273e";
        let group_id = 1;
        let result = move_to_group(env_uuid, group_id).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_batch_move_to_group() {
        crate::setup().await;
        let env_uuids = vec![
            "3dcd8228-120b-4ae7-b8d7-da7e6628269e".to_string(),
            "6359e3b8-25e9-470a-943e-6a17e62944d2".to_string(),
        ];
        let group_id = 1;
        let result = batch_move_to_group(env_uuids, group_id).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_modify_basic_info() {
        crate::setup().await;
        let uuid = "3dcd8228-120b-4ae7-b8d7-da7e6628269e";
        let name = "new-name";
        let description = Some("new-description".to_string());
        let result = modify_basic_info(uuid, name, description).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_modify_info() {
        crate::setup().await;
        let uuid = "02e8a4ba-dad5-40d9-b4ec-31d3334b09c8";
        let environment_uuid = "15dc4369-0fa7-49ff-8a63-9abd3dc1f375";
        let result = modify_info(
            uuid,
            environment_uuid,
            lp_models::environment::EnvironmentInfo {
                id: 0,
                fp_info: EnvironmentFingerprint {
                    id: Some(5),
                    browser: "12shdufshdf34".to_string(),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_delete() {
        crate::setup().await;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5";
        let env_uuid = "e5368907-d858-47e4-bfee-eddabbd36a56";
        let result = delete(user_uuid, env_uuid).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_batch_delete() {
        crate::setup().await;
        let user_uuid = "ac19b5cc-5a84-490d-b913-452ee71c52c7";
        let env_ids = vec!["316b654d-6d00-4a91-af15-a88e25e5a158".to_string()];
        let result = batch_delete(user_uuid, env_ids).await;
        println!("{:?}", result);
    }
}
