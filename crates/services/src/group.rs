use models::group::Group;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_id(id: i32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let group = Group::query_id(pool, id).await?;

    Ok(json!({
        "data": group,
    }))
}

pub async fn query(page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, groups) = Group::query_by_col(pool, "", "", page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": groups,
    }))
}

pub async fn create(payload: &Group) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Group::insert(pool, payload).await?;

    Ok(ok)
}

pub async fn modify(payload: &Group) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Group::update(pool, payload).await?;

    Ok(ok)
}

pub async fn delete(id: i32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Group::delete(pool, id).await?;

    Ok(ok)
}
