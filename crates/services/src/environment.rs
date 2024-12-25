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
    env_uuids: Vec<&str>,
    group_id: u32,
) -> Result<Vec<Value>, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let mut response = vec![];
    for env_id in env_uuids {
        let ok =
            models::environment::Environment::move_environment_to_group(pool, env_id, group_id)
                .await?;
        response.push(json!({
            "environment_id": env_id,
            "success": ok,
        }));
    }
    Ok(response)
}

pub async fn delete(user_uuid: &str, env_uuid: &str) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = models::environment::Environment::delete_and_move_to_trash(pool, user_uuid, env_uuid)
        .await?;

    Ok(ok)
}

pub async fn batch_delete(user_uuid: &str, env_ids: Vec<&str>) -> Result<Vec<bool>, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let mut response = vec![];
    for env_id in env_ids {
        let ok =
            models::environment::Environment::delete_and_move_to_trash(pool, env_id, user_uuid)
                .await?;
        response.push(ok);
    }
    Ok(response)
}
