use lp_models::environment_proxy_group::ProxyGroup;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_group_id(id: u32) -> Result<ProxyGroup, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;
    let proxy_group = ProxyGroup::query_proxy_group_by_id(pool, id).await?;

    Ok(proxy_group)
}

pub async fn query(user_uuid: &str, page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let (total, proxies) =
        ProxyGroup::query_all_proxy_groups_by_user_uuid(pool, user_uuid, page_num, page_size)
            .await?;

    Ok(json!({
        "total": total,
        "data": proxies,
    }))
}

pub async fn create(user_uuid: &str, payload: ProxyGroup) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let ok = ProxyGroup::insert_proxy_group(pool, user_uuid, &payload).await?;

    Ok(ok)
}

pub async fn update(id: u32, payload: ProxyGroup) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let ok = ProxyGroup::update_proxy_group(pool, id, &payload).await?;

    Ok(ok)
}

pub async fn delete(id: u32) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let ok = ProxyGroup::delete_proxy_group(pool, id).await?;

    Ok(ok)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_by_group_id() {
        crate::setup().await;
        let result = query_by_group_id(1).await;
        let proxy_group = result.unwrap();
        println!("{:?}", proxy_group);
    }

    #[tokio::test]
    async fn test_query() {
        crate::setup().await;
        let result = query("3cfb0bc6-7b48-498a-935a-90ce561e40a5", 1, 10).await;
        let value: Value = result.unwrap();
        println!("{:?}", value);
    }

    #[tokio::test]
    async fn test_create() {
        crate::setup().await;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5";
        let proxy_group = ProxyGroup {
            ..Default::default()
        };
        let result = create(user_uuid, proxy_group).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_update() {
        crate::setup().await;
        let proxy_group = ProxyGroup {
            name:"abc".to_string(),
            ..Default::default()
        };
        let result = update(1, proxy_group).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_delete() {
        crate::setup().await;
        let result = delete(1).await;
        println!("{:?}", result);
    }
}
