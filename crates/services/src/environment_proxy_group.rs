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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_by_group_id() {
        let result = query_by_group_id(1).await;
        let _proxy_group = result.unwrap();
    }

    #[tokio::test]
    async fn test_query() {
        let result = query("user-uuid", 1, 10).await;
        assert!(result.is_ok());
        let value: Value = result.unwrap();
        assert!(value["total"].as_u64().is_some());
        assert!(value["data"].as_array().is_some());
    }

    #[tokio::test]
    async fn test_create() {
        let proxy_group = ProxyGroup {
            ..Default::default()
        };
        let result = create(proxy_group).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_update() {
        let proxy_group = ProxyGroup {
            ..Default::default()
        };
        let result = update(1, proxy_group).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_delete() {
        let result = delete(1).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}
