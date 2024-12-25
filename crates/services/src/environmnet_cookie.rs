use models::environmnet_cookie::EnvironmentCookie;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_id(id: u32) -> Result<EnvironmentCookie, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let cookie = EnvironmentCookie::query_by_id(pool, id).await?;

    Ok(cookie)
}

pub async fn query(
    environmnet_uuid: &str,
    page_num: u32,
    page_size: u32,
) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, cookies) =
        EnvironmentCookie::query_by_environment_uuid(pool, environmnet_uuid, page_num, page_size)
            .await?;

    Ok(json!({
        "total": total,
        "data": cookies,
    }))
}

pub async fn create(payload: &EnvironmentCookie) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentCookie::insert(pool, payload).await?;

    Ok(ok)
}

pub async fn modify(payload: &EnvironmentCookie) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentCookie::update(pool, payload).await?;

    Ok(ok)
}

pub async fn delete(id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentCookie::delete(pool, id).await?;

    Ok(ok)
}
