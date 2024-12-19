use models::fingerprint::Fingerprint;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_id(id: i32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let fingerprint = Fingerprint::query_by_id(pool, id).await?;

    Ok(json!({
        "data": fingerprint,
    }))
}

pub async fn query(page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, groups) = Fingerprint::query_by_col(pool, "", "", page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": groups,
    }))
}

pub async fn create(payload: &Fingerprint) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Fingerprint::insert(pool, payload).await?;

    Ok(ok)
}

pub async fn modify(payload: &Fingerprint) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Fingerprint::update(pool, payload).await?;

    Ok(ok)
}

pub async fn delete(id: i32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Fingerprint::delete(pool, id).await?;

    Ok(ok)
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    pub async fn test_query() {
        crate::setup().await;
        let ok = super::query(1, 10).await.unwrap();

        println!("{:?}", ok);
    }

    #[tokio::test]
    pub async fn test_query_by_id() {
        crate::setup().await;
        let ok = super::query_by_id(2).await.unwrap();

        println!("{:?}", ok);
    }

    #[tokio::test]
    pub async fn test_create() {
        crate::setup().await;
        let ok = super::create(&models::fingerprint::Fingerprint {
            ..Default::default()
        })
        .await
        .unwrap();

        println!("{:?}", ok);
    }

    #[tokio::test]
    pub async fn test_modify() {
        crate::setup().await;
        let ok = super::modify(&models::fingerprint::Fingerprint {
            id: Some(2),
            ua: "abc".to_string(),
            ..Default::default()
        })
        .await
        .unwrap();

        println!("{:?}", ok);
    }

    #[tokio::test]
    pub async fn test_delete() {
        crate::setup().await;
        let ok = super::delete(2).await.unwrap();

        println!("{:?}", ok);
    }
}
