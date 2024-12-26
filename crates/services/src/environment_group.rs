use models::environment_group::EnvironmentGroup;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_id(id: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let group = EnvironmentGroup::query_by_id(pool, id).await?;

    Ok(json!({
        "data": group,
    }))
}

pub async fn query(user_uuid: &str, page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, groups) =
        EnvironmentGroup::query_groups_by_user_uuid(pool, user_uuid, page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": groups,
    }))
}

pub async fn create(payload: &EnvironmentGroup) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentGroup::insert(pool, payload).await?;

    Ok(ok)
}

pub async fn modify(id: u32, payload: &EnvironmentGroup) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentGroup::update(pool, id, payload).await?;

    Ok(ok)
}

pub async fn delete(group_id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentGroup::delete(pool, group_id).await?;

    Ok(ok)
}
