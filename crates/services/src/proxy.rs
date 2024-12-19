use models::proxies::Proxy;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_id(id: i32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let proxy = Proxy::query_id(pool, id).await?;

    Ok(json!({
        "data": proxy,
    }))
}

pub async fn query(page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, proxies) = Proxy::query_by_col(pool, "", "", page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": proxies,
    }))
}

pub async fn create(payload: &Proxy) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Proxy::insert(pool, payload).await?;

    Ok(ok)
}

pub async fn modify(payload: &Proxy) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Proxy::update(pool, payload).await?;

    Ok(ok)
}

pub async fn delete(id: i32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Proxy::delete(pool, id).await?;

    Ok(ok)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create() {
        crate::setup().await;

        let ok = super::create(&Proxy {
            ..Default::default()
        })
        .await
        .unwrap();
        println!("{:?}", ok);
    }

    #[tokio::test]
    async fn test_query_by_id() {
        crate::setup().await;
        let proxy = super::query_by_id(1).await.unwrap();
        println!("{:?}", proxy);
    }

    #[tokio::test]
    async fn test_query() {
        crate::setup().await;
        let proxy = super::query(1, 10).await.unwrap();
        println!("{:?}", proxy);
    }

    #[tokio::test]
    async fn test_modify() {
        crate::setup().await;

        let ok = super::modify(&Proxy {
            id: 1,
            value: "abc".to_string(),
            ..Default::default()
        })
        .await
        .unwrap();

        println!("{:?}", ok);
    }

    #[tokio::test]
    async fn test_() {
        crate::setup().await;

        let ok = super::delete(1).await.unwrap();
        println!("{:?}", ok);
    }
}
