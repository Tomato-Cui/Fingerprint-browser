use models::fingerprint::Fingerprint;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_id(user_id: u32, id: u32) -> Result<Fingerprint, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let fingerprint = Fingerprint::query_by_id(pool, user_id, id).await?;
    Ok(fingerprint)
}

pub async fn default() -> Result<Fingerprint, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let fingerprint = Fingerprint::default_fingerprint(pool).await?;

    Ok(fingerprint)
}

pub async fn query(user_id: u32, page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, groups) =
        Fingerprint::query_by_col(pool, user_id, "", "", page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": groups,
    }))
}

pub async fn create(user_id: u32, payload: &Fingerprint) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Fingerprint::insert(pool, user_id, payload).await?;

    Ok(ok)
}

pub async fn modify(user_id: u32, payload: &Fingerprint) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Fingerprint::update(pool, user_id, payload).await?;

    Ok(ok)
}

pub async fn delete(user_id: u32, id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Fingerprint::delete(pool, user_id, id).await?;

    Ok(ok)
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    pub async fn test_query() {
        crate::setup().await;
        let ok = super::query(1, 1, 10).await.unwrap();

        println!("{:?}", ok);
    }

    #[tokio::test]
    pub async fn test_query_by_id() {
        crate::setup().await;
        let ok = super::query_by_id(1, 2).await.unwrap();

        println!("{:?}", ok);
    }

    #[tokio::test]
    pub async fn test_create() {
        crate::setup().await;
        let ok = super::create(
            1,
            &models::fingerprint::Fingerprint {
                ..Default::default()
            },
        )
        .await
        .unwrap();

        println!("{:?}", ok);
    }

    #[tokio::test]
    pub async fn test_modify() {
        crate::setup().await;
        let ok = super::modify(
            1,
            &models::fingerprint::Fingerprint {
                id: Some(2),
                ua: "abc".to_string(),
                ..Default::default()
            },
        )
        .await
        .unwrap();

        println!("{:?}", ok);
    }

    #[tokio::test]
    pub async fn test_delete() {
        crate::setup().await;
        let ok = super::delete(1, 2).await.unwrap();

        println!("{:?}", ok);
    }
}
