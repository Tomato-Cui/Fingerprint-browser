use models::environment_fingerprint::EnvironmentFingerprint;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_id(user_uuid: &str, id: u32) -> Result<EnvironmentFingerprint, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let fingerprint = EnvironmentFingerprint::query_by_id(pool, user_uuid, id).await?;
    Ok(fingerprint)
}

pub async fn query(user_uuid: &str, page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, groups) =
        EnvironmentFingerprint::query_by_user_uuid(pool, user_uuid, page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": groups,
    }))
}

pub async fn create(
    user_uuid: &str,
    payload: &EnvironmentFingerprint,
) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentFingerprint::insert(pool, user_uuid, payload).await?;

    Ok(ok)
}

pub async fn modify(
    user_uuid: &str,
    payload: &EnvironmentFingerprint,
) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentFingerprint::update(pool, user_uuid, payload).await?;

    Ok(ok)
}

pub async fn delete(user_uuid: &str, id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentFingerprint::delete(pool, user_uuid, id).await?;

    Ok(ok)
}
