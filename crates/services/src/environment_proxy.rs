use models::environment_proxies::Proxy;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_id(user_uuid: &str, id: u32) -> Result<Proxy, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let proxy = Proxy::query_proxy_by_id(pool, id, user_uuid).await?;

    Ok(proxy)
}

pub async fn query_by_uuid(
    user_uuid: &str,
    page_num: u32,
    page_size: u32,
) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let (total, proxies) =
        Proxy::query_proxy_groups_by_user_uuid(pool, user_uuid, page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": proxies,
    }))
}

pub async fn query(user_uuid: &str, page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, proxies) =
        Proxy::query_proxies_by_user_uuid(pool, user_uuid, page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": proxies,
    }))
}

pub async fn create(user_uuid: &str, mut payload: Proxy) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    payload.user_uuid = Some(user_uuid.to_string());

    let ok = Proxy::insert_proxy(pool, &payload).await?;

    Ok(ok)
}

pub async fn update(user_uuid: &str, id: u32, payload: Proxy) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Proxy::update_proxy(pool, id, user_uuid, &payload).await?;

    Ok(ok)
}

pub async fn move_to_proxy_group(proxy_id: u32, proxy_group_id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Proxy::update_proxy_group_for_proxy(pool, proxy_id, proxy_group_id).await?;

    Ok(ok)
}

pub async fn delete(user_uuid: &str, id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Proxy::delete_proxy(pool, id, user_uuid).await?;

    Ok(ok)
}
