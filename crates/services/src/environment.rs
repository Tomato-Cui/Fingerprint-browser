use models::environment::Environment;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_uuid(uuid: &str) -> Result<Environment, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let environment = models::environment::Environment::query_by_uuid(pool, uuid).await?;

    Ok(environment)
}

pub async fn query(user_uuid: &str, page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, environment) = models::environment::Environment::query_environments_by_user_uuid(
        pool, user_uuid, page_num, page_size,
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
    let pool = states::database::get_database_pool()?;
    let (total, environment) = models::environment::Environment::query_environments_by_group_id(
        pool, group_id, page_num, page_size,
    )
    .await?;

    Ok(json!({
        "total": total,
        "data": environment,
    }))
}

pub async fn create(user_uuid: &str, mut payload: Environment) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    payload.user_uuid = user_uuid.to_string();
    payload.uuid = Some(commons::encryption::uuid());
    println!("{:?}", payload);

    let ok = models::environment::Environment::insert(pool, &payload).await?;
    Ok(ok)
}

pub async fn create_batch(
    user_uuid: &str,
    payload: Vec<Environment>,
) -> Result<Vec<Value>, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let mut response = vec![];
    for mut env_ in payload {
        env_.uuid = Some(commons::encryption::uuid());
        env_.user_uuid = user_uuid.to_string();

        let ok = models::environment::Environment::insert(pool, &env_).await?;
        response.push(json!({
            "name": env_.name,
            "success": ok
        }));
    }

    Ok(response)
}

pub async fn move_to_group(env_uuid: &str, group_id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let ok = models::environment::Environment::move_environment_to_group(pool, env_uuid, group_id)
        .await?;

    Ok(ok)
}

pub async fn batch_move_to_group(
    env_uuids: Vec<String>,
    group_id: u32,
) -> Result<Vec<Value>, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let mut response = vec![];
    for env_id in env_uuids {
        let ok =
            models::environment::Environment::move_environment_to_group(pool, &env_id, group_id)
                .await?;
        response.push(json!({
            "environment_id": env_id,
            "success": ok,
        }));
    }
    Ok(response)
}

pub async fn modify_info(
    uuid: &str,
    name: &str,
    description: Option<String>,
) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok =
        models::environment::Environment::modify_basic_info(pool, uuid, name, description).await?;

    Ok(ok)
}

pub async fn delete(user_uuid: &str, env_uuid: &str) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = models::environment::Environment::delete_and_move_to_trash(pool, env_uuid, user_uuid)
        .await?;

    Ok(ok)
}

pub async fn batch_delete(
    user_uuid: &str,
    env_ids: Vec<String>,
) -> Result<Vec<bool>, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let mut response = vec![];
    for env_id in env_ids {
        let ok =
            models::environment::Environment::delete_and_move_to_trash(pool, &env_id, user_uuid)
                .await?;
        response.push(ok);
    }
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_by_uuid() {
        crate::setup().await;
        let uuid = "9796eae9-1263-412e-9651-5ef1f13882eb";
        let result = query_by_uuid(uuid).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_query() {
        crate::setup().await;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5";
        let page_num = 1;
        let page_size = 10;
        let result = query(user_uuid, page_num, page_size).await;
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
    async fn test_create() {
        crate::setup().await;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5".to_string();
        let payload = Environment {
            name: "123".to_string(),
            ..Default::default()
        };

        let result = create(&user_uuid, payload).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_create_batch() {
        crate::setup().await;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5".to_string();
        let payload = vec![
            Environment {
                ..Default::default()
            },
            Environment {
                ..Default::default()
            },
        ];
        let result = create_batch(&user_uuid, payload).await;
        assert!(result.is_ok());
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
    async fn test_modify_info() {
        crate::setup().await;
        let uuid = "3dcd8228-120b-4ae7-b8d7-da7e6628269e";
        let name = "new-name";
        let description = Some("new-description".to_string());
        let result = modify_info(uuid, name, description).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_delete() {
        crate::setup().await;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5";
        let env_uuid = "2b7e8675-cebe-45d1-81c2-5fb0aa1d273e";
        let result = delete(user_uuid, env_uuid).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_batch_delete() {
        let user_uuid = "test-user-uuid";
        let env_ids = vec!["test-env-uuid1".to_string(), "test-env-uuid2".to_string()];
        let result = batch_delete(user_uuid, env_ids).await;
        assert!(result.is_ok());
    }
}
