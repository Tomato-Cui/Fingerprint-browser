use models::environment_proxy_group::ProxyGroup;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_group_id(id: u32) -> Result<ProxyGroup, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let proxy_group = ProxyGroup::query_proxy_group_by_id(pool, id).await?;

    Ok(proxy_group)
}

pub async fn query(user_uuid: &str, page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, proxies) =
        ProxyGroup::query_all_proxy_groups_by_user_uuid(pool, user_uuid, page_num, page_size)
            .await?;

    Ok(json!({
        "total": total,
        "data": proxies,
    }))
}

pub async fn create(payload: ProxyGroup) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = ProxyGroup::insert_proxy_group(pool, &payload).await?;

    Ok(ok)
}

pub async fn update(id: u32, payload: ProxyGroup) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = ProxyGroup::update_proxy_group(pool, id, &payload).await?;

    Ok(ok)
}

pub async fn delete(id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = ProxyGroup::delete_proxy_group(pool, id).await?;

    Ok(ok)
}
