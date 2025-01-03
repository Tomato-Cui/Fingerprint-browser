use models::environment_fingerprint::EnvironmentFingerprint;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_id(user_uuid: &str, id: u32) -> Result<EnvironmentFingerprint, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let fingerprint = EnvironmentFingerprint::query_by_id(pool, user_uuid, id).await?;
    Ok(fingerprint)
}

pub async fn query(user_uuid: &str, page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, groups) =
        EnvironmentFingerprint::query_by_user_uuid(pool, user_uuid, page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": groups,
    }))
}

pub async fn create(
    user_uuid: &str,
    payload: &EnvironmentFingerprint,
) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentFingerprint::insert(pool, user_uuid, payload).await?;

    Ok(ok > 0)
}

pub async fn modify(
    user_uuid: &str,
    id: u32,
    payload: &EnvironmentFingerprint,
) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentFingerprint::update(pool, id, user_uuid, payload).await?;

    Ok(ok)
}

pub async fn delete(user_uuid: &str, id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentFingerprint::delete(pool, user_uuid, id).await?;

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
    async fn test_query() {
        crate::setup().await;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5";
        let page_num = 1;
        let page_size = 10;
        let result = query(user_uuid, page_num, page_size).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_create() {
        crate::setup().await;
        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5";
        let payload = EnvironmentFingerprint {
            ..Default::default()
        };
        let result = create(user_uuid, &payload).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_modify() {
        crate::setup().await;
        let user_uuid = "02e8a4ba-dad5-40d9-b4ec-31d3334b09c8";
        let id = 5;
        let payload = EnvironmentFingerprint {
            browser: "abc".to_string(),
            ..Default::default()
        };
        let result = modify(user_uuid, id, &payload).await;
        println!("{:?}", result);
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
