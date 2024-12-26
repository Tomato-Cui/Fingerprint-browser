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
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_by_environment_uuid() {
        let environment_uuid = "test-uuid";
        let result = query_by_environment_uuid(environment_uuid).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_query() {
        let user_uuid = "test-user-uuid";
        let page_num = 1;
        let page_size = 10;
        let result = query(user_uuid, page_num, page_size).await;
        assert!(result.is_ok());
        let value: Value = result.unwrap();
        assert!(value.get("total").is_some());
        assert!(value.get("data").is_some());
    }

    #[tokio::test]
    async fn test_recover() {
        let user_uuid = "test-user-uuid";
        let environment_uuid = "test-environment-uuid";
        let result = recover(user_uuid, environment_uuid).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_recovers() {
        let user_uuid = "test-user-uuid";
        let environment_uuids = vec!["test-environment-uuid1", "test-environment-uuid2"];
        let result = recovers(user_uuid, environment_uuids).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_recover_all() {
        let user_uuid = "test-user-uuid";
        let result = recover_all(user_uuid).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_batch_delete_again() {
        let environment_uuids = vec!["test-environment-uuid1", "test-environment-uuid2"];
        let result = batch_delete_again(environment_uuids).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_clean() {
        let user_uuid = "test-user-uuid";
        let result = clean(user_uuid).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}
