use models::proxies::Proxy;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_id(user_id: u32, id: u32) -> Result<Proxy, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let proxy = Proxy::query_id(pool, user_id, id).await?;

    Ok(proxy)
}

pub async fn query_by_env_id(user_id: u32, environment_id: u32) -> Result<Proxy, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let proxy = Proxy::query_by_environment_id(pool, user_id, environment_id).await?;

    Ok(proxy)
}

pub async fn query(user_id: u32, page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, proxies) = Proxy::query_by_col(pool, user_id, "", "", page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": proxies,
    }))
}

pub async fn create(user_id: u32, payload: &Proxy) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Proxy::insert(pool, user_id, payload).await?;

    Ok(ok)
}

pub async fn modify(user_id: u32, payload: &Proxy) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Proxy::update(pool, user_id, payload).await?;

    Ok(ok)
}

pub async fn delete(user_id: u32, id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Proxy::delete(pool, user_id, id).await?;

    Ok(ok)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create() {
        crate::setup().await;

        let ok = super::create(
            1,
            &Proxy {
                ..Default::default()
            },
        )
        .await
        .unwrap();
        println!("{:?}", ok);
    }

    #[tokio::test]
    async fn test_query_by_id() {
        crate::setup().await;
        let proxy = super::query_by_id(1, 1).await.unwrap();
        println!("{:?}", proxy);
    }

    #[tokio::test]
    async fn test_query() {
        crate::setup().await;
        let proxy = super::query(1, 1, 10).await.unwrap();
        println!("{:?}", proxy);
    }

    #[tokio::test]
    async fn test_modify() {
        crate::setup().await;

        let ok = super::modify(
            1,
            &Proxy {
                id: 1,
                value: "abc".to_string(),
                ..Default::default()
            },
        )
        .await
        .unwrap();

        println!("{:?}", ok);
    }

    #[tokio::test]
    async fn test_() {
        crate::setup().await;

        let ok = super::delete(1, 1).await.unwrap();
        println!("{:?}", ok);
    }
}
