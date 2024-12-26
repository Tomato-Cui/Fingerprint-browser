use models::environment_account::EnvironmentAccount;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_id(id: u32) -> Result<EnvironmentAccount, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let account = EnvironmentAccount::query_by_id(pool, id).await?;

    Ok(account)
}

pub async fn query(user_uuid: &str, page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, accounts) =
        EnvironmentAccount::query_by_environment_uuid(pool, user_uuid, page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": accounts,
    }))
}

pub async fn create(payload: &EnvironmentAccount) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentAccount::insert(pool, payload).await?;

    Ok(ok)
}

pub async fn modify(payload: &EnvironmentAccount) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentAccount::update(pool, payload).await?;

    Ok(ok)
}

pub async fn delete(id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentAccount::delete(pool, id).await?;

    Ok(ok)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_by_id() {
        crate::setup().await;

        let result = query_by_id(1).await;
        assert!(result.is_ok());
        let account = result.unwrap();

        println!("{:?}", account)
    }

    #[tokio::test]
    async fn test_query() {
        crate::setup().await;

        let result = query("some-uuid", 1, 10).await;
        assert!(result.is_ok());
        let value = result.unwrap();
        println!("{:?}", value)
    }

    #[tokio::test]
    async fn test_create() {
        crate::setup().await;

        let payload = EnvironmentAccount {
            platform: "windows".to_string(),
            platform_url: "http://baidu.com".to_string(),
            platform_account: "liusjjkk".to_string(),
            platform_password: "String".to_string(),
            environment_uuid: "shdfj".to_string(),
            user_uuid: "hdsjf".to_string(),
            ..Default::default()
        };
        let result = create(&payload).await.unwrap();
        println!("{:?}", result)
    }

    #[tokio::test]
    async fn test_modify() {
        crate::setup().await;

        let payload = EnvironmentAccount {
            platform: "windows".to_string(),
            platform_url: "http://baidu.com".to_string(),
            platform_account: "liusjjkk".to_string(),
            platform_password: "String".to_string(),
            environment_uuid: "shdfj".to_string(),
            user_uuid: "hdsjf".to_string(),
            ..Default::default()
        };
        let result = modify(&payload).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_delete() {
        crate::setup().await;

        let result = delete(1).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}
