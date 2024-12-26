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
        EnvironmentTransferHistory::query_environments_by_from_user_uuid(
            pool, user_uuid, page_num, page_size,
        )
        .await?;

    Ok(json!({
        "total": total,
        "data": transfer_histories,
    }))
}

pub async fn create(
    payloads: Vec<EnvironmentTransferHistory>,
) -> Result<HashMap<String, bool>, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let mut res = HashMap::new();
    for environment_transfer_history in payloads {
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
        let environment_uuid = "test-env-uuid";
        let result = query_by_id(environment_uuid).await;
        assert!(result.is_ok());
        let transfer_history = result.unwrap();
        assert_eq!(transfer_history.environment_uuid, environment_uuid);
    }

    #[tokio::test]
    async fn test_query() {
        let user_uuid = "test-user-uuid";
        let page_num = 1;
        let page_size = 10;
        let result = query(user_uuid, page_num, page_size).await;
        assert!(result.is_ok());
        let value: Value = result.unwrap();
        assert!(value["total"].is_number());
        assert!(value["data"].is_array());
    }

    #[tokio::test]
    async fn test_create() {
        let payloads = vec![EnvironmentTransferHistory {
            ..Default::default()
        }];
        let result = create(payloads).await;
        assert!(result.is_ok());
        let res: HashMap<String, bool> = result.unwrap();
        assert!(res.contains_key("test-env-uuid"));
        assert!(res["test-env-uuid"]);
    }

    #[tokio::test]
    async fn test_modify() {
        let payload = EnvironmentTransferHistory {
            ..Default::default()
        };
        let result = modify(&payload).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_delete() {
        let user_uuid = "test-user-uuid";
        let environment_uuid = "test-env-uuid";
        let result = delete(user_uuid, environment_uuid).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}
