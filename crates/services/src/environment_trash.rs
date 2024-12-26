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
        crate::setup().await;
        let environment_uuid = "a8bb7d2e-09ec-436b-aa51-2e5a6424acbd";
        let result = query_by_environment_uuid(environment_uuid).await;
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
    async fn test_recover() {
        crate::setup().await;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5";
        let environment_uuid = "e5368907-d858-47e4-bfee-eddabbd36a56";
        let result = recover(user_uuid, environment_uuid).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_recovers() {
        crate::setup().await;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5";
        let environment_uuids = vec![
            "a8bb7d2e-09ec-436b-aa51-2e5a6424acbd",
            "e5368907-d858-47e4-bfee-eddabbd36a56",
        ];
        let result = recovers(user_uuid, environment_uuids).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_recover_all() {
        crate::setup().await;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5";
        let result = recover_all(user_uuid).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_batch_delete_again() {
        crate::setup().await;
        let environment_uuids = vec![
            "a8bb7d2e-09ec-436b-aa51-2e5a6424acbd",
            "e5368907-d858-47e4-bfee-eddabbd36a56",
        ];
        let result = batch_delete_again(environment_uuids).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_clean() {
        crate::setup().await;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5";
        let result = clean(user_uuid).await;
        println!("{:?}", result);
    }
}
