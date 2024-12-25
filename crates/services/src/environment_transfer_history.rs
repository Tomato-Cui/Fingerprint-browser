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
