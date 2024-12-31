use models::{environment::Environment, environment_proxies::Proxy};
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
        Proxy::query_proxies_by_user_uuid(pool, user_uuid, page_num, page_size).await?;

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

    Ok(ok > 1)
}

pub async fn update(user_uuid: &str, id: u32, payload: Proxy) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Proxy::update_proxy(pool, id, user_uuid, &payload).await?;

    Ok(ok)
}

pub async fn update_proxy(environmnet_uuid: &str, payload: Proxy) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Environment::modify_proxy(pool, environmnet_uuid, &payload).await?;

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

pub async fn batch_delete(user_uuid: &str, ids: Vec<u32>) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Proxy::delete_proxys(pool, ids, user_uuid).await?;

    Ok(ok)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_by_id() {
        crate::setup().await;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5";
        let id = 1;
        let result = query_by_id(user_uuid, id).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_query_by_uuid() {
        crate::setup().await;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5";
        let page_num = 1;
        let page_size = 10;
        let result = query_by_uuid(user_uuid, page_num, page_size).await;
        assert!(result.is_ok());
        let value: Value = result.unwrap();
        println!("{:?}", value);
    }

    #[tokio::test]
    async fn test_query() {
        crate::setup().await;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5";
        let page_num = 1;
        let page_size = 10;
        let result = query(user_uuid, page_num, page_size).await;
        assert!(result.is_ok());
        let value: Value = result.unwrap();
        println!("{:?}", value);
    }

    #[tokio::test]
    async fn test_create() {
        crate::setup().await;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5";
        let payload = Proxy {
            ..Default::default()
        };
        let result = create(user_uuid, payload).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_update() {
        crate::setup().await;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5";
        let id = 1;
        let payload = Proxy {
            kind: "abc".to_string(),
            ..Default::default()
        };
        let result = update(user_uuid, id, payload).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_move_to_proxy_group() {
        crate::setup().await;
        let proxy_id = 1;
        let proxy_group_id = 2;
        let result = move_to_proxy_group(proxy_id, proxy_group_id).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_delete() {
        crate::setup().await;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5";
        let id = 1;
        let result = delete(user_uuid, id).await;
        println!("{:?}", result);
    }
}
