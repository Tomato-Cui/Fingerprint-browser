use std::collections::HashMap;

use models::environment_transfer_history::EnvironmentTransferHistory;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_id(
    environment_uuid: &str,
) -> Result<EnvironmentTransferHistory, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let transfer_history = EnvironmentTransferHistory::query_transfer_history_by_environment_uuid(
        pool,
        environment_uuid,
    )
    .await?;

    Ok(transfer_history)
}

pub async fn query(user_uuid: &str, page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, transfer_histories) =
        EnvironmentTransferHistory::query_environments_by_user_uuid(
            pool, user_uuid, page_num, page_size,
        )
        .await?;

    Ok(json!({
        "total": total,
        "data": transfer_histories,
    }))
}

pub async fn create(
    user_uuid: &str,
    user_email: &str,
    environment_uuids: Vec<String>,
) -> Result<HashMap<String, bool>, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let mut res = HashMap::new();

    let current_user_uuid = models::user::User::query_uuid_by_email(pool, user_email).await?;

    for environment_uuid in environment_uuids {
        let environment_transfer_history = EnvironmentTransferHistory {
            environment_uuid,
            from_user_uuid: user_uuid.to_string(),
            to_user_uuid: current_user_uuid.clone(),
            ..Default::default()
        };

        let ok = EnvironmentTransferHistory::insert_transfer_history(
            pool,
            &environment_transfer_history,
        )
        .await?;

        res.insert(environment_transfer_history.environment_uuid, ok);
    }

    Ok(res)
}

pub async fn modify(payload: &EnvironmentTransferHistory) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentTransferHistory::update_transfer_history(pool, payload).await?;

    Ok(ok)
}

pub async fn delete(user_uuid: &str, environment_uuid: &str) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentTransferHistory::delete_transfer_history(pool, user_uuid, environment_uuid)
        .await?;

    Ok(ok)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_by_id() {
        crate::setup().await;
        let environment_uuid = "e5368907-d858-47e4-bfee-eddabbd36a56";
        let result = query_by_id(environment_uuid).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_query() {
        crate::setup().await;
        let user_uuid = "2501e251-d3bd-4852-a2a3-088046fd2119";
        let page_num = 1;
        let page_size = 10;
        let result = query(user_uuid, page_num, page_size).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_create() {
        crate::setup().await;
        let environment_uuids = vec!["60799ae4-5923-48e1-abf4-334c93fb9ecc".to_string()];

        let result = create("2501e251-d3bd-4852-a2a3-088046fd2119", "this", environment_uuids).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_modify() {
        crate::setup().await;
        let payload = EnvironmentTransferHistory {
            ..Default::default()
        };
        let result = modify(&payload).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_delete() {
        crate::setup().await;
        let user_uuid = "9d4250ca-e1a5-40b1-ba9f-a0b6d98c1db8";
        let environment_uuid = "e5368907-d858-47e4-bfee-eddabbd36a56";
        let result = delete(user_uuid, environment_uuid).await;
        println!("{:?}", result);
    }
}
