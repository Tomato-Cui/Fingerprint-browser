use models::environment_account::EnvironmentAccount;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_id(id: u32) -> Result<EnvironmentAccount, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let account = EnvironmentAccount::query_by_id(pool, id).await?;

    Ok(account)
}

pub async fn query(user_uuid: &str, page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, accounts) =
        EnvironmentAccount::query_by_environment_uuid(pool, user_uuid, page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": accounts,
    }))
}

pub async fn create(payload: &EnvironmentAccount) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentAccount::insert(pool, payload).await?;

    Ok(ok)
}

pub async fn modify(payload: &EnvironmentAccount) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentAccount::update(pool, payload).await?;

    Ok(ok)
}

pub async fn delete(id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentAccount::delete(pool, id).await?;

    Ok(ok)
}
