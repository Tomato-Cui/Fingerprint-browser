use models::group::Group;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_id(user_id: u32, id: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let group = Group::query_id(pool, id, user_id).await?;

    Ok(json!({
        "data": group,
    }))
}

pub async fn query(user_id: u32, page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, groups) = Group::query_by_col(pool, user_id, "", "", page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": groups,
    }))
}

pub async fn create(user_id: u32, payload: &Group) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Group::insert(pool, user_id, payload).await?;

    Ok(ok)
}

pub async fn modify(user_id: u32, payload: &Group) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Group::update(pool, user_id, payload).await?;

    Ok(ok)
}

pub async fn grant_user(user_id: u32, group_id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Group::update_grant_user(pool, user_id, group_id).await?;

    Ok(ok)
}

pub async fn delete(user_id: u32, group_id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Group::delete(pool, user_id, group_id).await?;

    Ok(ok)
}
