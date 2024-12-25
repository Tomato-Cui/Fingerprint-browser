use models::environment_transfer_history::EnvironmentTransferHistory;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_id(id: u32) -> Result<EnvironmentTransferHistory, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let transfer_history =
        EnvironmentTransferHistory::query_transfer_history_by_id(pool, id).await?;

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

pub async fn create(payload: &EnvironmentTransferHistory) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentTransferHistory::insert_transfer_history(pool, payload).await?;

    Ok(ok)
}

pub async fn modify(payload: &EnvironmentTransferHistory) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentTransferHistory::update_transfer_history(pool, payload).await?;

    Ok(ok)
}

pub async fn delete(id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentTransferHistory::delete_transfer_history(pool, id).await?;

    Ok(ok)
}
