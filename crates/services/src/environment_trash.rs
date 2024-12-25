use models::environment::Environment;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_environment_uuid(
    environmnet_uuid: &str,
) -> Result<Environment, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let  environment =
        models::environment_trash::EnvironmentTrash::query_deleted_environments_by_environment_uuid(
            pool, environmnet_uuid
        )
        .await?;
    Ok(environment)
}

pub async fn query(user_uuid: &str, page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, environment) =
        models::environment_trash::EnvironmentTrash::query_deleted_environments_by_user_uuid(
            pool, user_uuid, page_num, page_size,
        )
        .await?;

    Ok(json!({
        "total": total,
        "data": environment,
    }))
}

pub async fn recover(user_uuid: &str, environmnet_uuid: &str) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let ok = models::environment_trash::EnvironmentTrash::restore_deleted_environment(
        pool,
        user_uuid,
        environmnet_uuid,
    )
    .await?;
    Ok(ok)
}

pub async fn recovers(user_uuid: &str, environmnet_uuids: Vec<&str>) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let ok = models::environment_trash::EnvironmentTrash::restore_deleted_environments(
        pool,
        user_uuid,
        &environmnet_uuids,
    )
    .await?;

    Ok(ok)
}

pub async fn recover_all(user_uuid: &str) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok =
        models::environment_trash::EnvironmentTrash::restore_all_deleted_environments_by_user_uuid(
            pool, user_uuid,
        )
        .await?;
    Ok(ok)
}

pub async fn batch_delete_again(env_uuids: Vec<&str>) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let ok = models::environment_trash::EnvironmentTrash::permanently_delete_environments(
        pool, &env_uuids,
    )
    .await?;

    Ok(ok)
}

pub async fn clean(user_uuid: &str) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok =
        models::environment_trash::EnvironmentTrash::permanently_delete_all_deleted_environments_by_user_uuid(pool, user_uuid)
            .await?;

    Ok(ok)
}
