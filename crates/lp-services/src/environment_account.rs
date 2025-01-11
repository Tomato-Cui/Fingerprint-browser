use lp_models::environment_account::EnvironmentAccount;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_id(id: u32) -> Result<EnvironmentAccount, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;
    let account = EnvironmentAccount::query_by_id(pool, id).await?;

    Ok(account)
}

pub async fn query(user_uuid: &str, page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let (total, accounts) =
        EnvironmentAccount::query_by_user_uuid(pool, user_uuid, page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": accounts,
    }))
}

pub async fn account_query_current_environment(
    environment_uuid: &str,
    page_num: u32,
    page_size: u32,
) -> Result<Value, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let (total, accounts) =
        EnvironmentAccount::query_by_environment_uuid(pool, environment_uuid, page_num, page_size)
            .await?;

    Ok(json!({
        "total": total,
        "data": accounts,
    }))
}

pub async fn create(
    user_uuid: &str,
    mut payload: EnvironmentAccount,
) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;
    payload.user_uuid = Some(user_uuid.to_string());

    let ok = EnvironmentAccount::insert(pool, &payload).await?;

    Ok(ok)
}

pub async fn modify(payload: &EnvironmentAccount) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let ok = EnvironmentAccount::update(pool, payload).await?;

    Ok(ok)
}

pub async fn delete(id: u32) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let ok = EnvironmentAccount::delete(pool, id).await?;

    Ok(ok)
}

pub async fn batch_delete(ids: Vec<u32>) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let ok = EnvironmentAccount::deletes(pool, ids).await?;

    Ok(ok)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_by_id() {
        crate::setup().await;

        let result = query_by_id(1).await;

        println!("{:?}", result)
    }

    #[tokio::test]
    async fn test_query() {
        crate::setup().await;

        let result = query("d3129a09-5473-4b1e-915b-bba0af78d752", 1, 10).await;
        println!("{:?}", result)
    }

    #[tokio::test]
    async fn test_create() {
        crate::setup().await;

        let user_uuid = "3cfb0bc6-7b48-498a-935a-90ce561e40a5".to_string();
        let payload = EnvironmentAccount {
            platform: "windows".to_string(),
            platform_url: "http://baidu.com".to_string(),
            platform_account: "liusjjkk".to_string(),
            platform_password: "String".to_string(),
            environment_uuid: "3dcd8228-120b-4ae7-b8d7-da7e6628269e".to_string(),
            ..Default::default()
        };
        let result = create(&user_uuid, payload).await.unwrap();
        println!("{:?}", result)
    }

    #[tokio::test]
    async fn test_modify() {
        crate::setup().await;

        let payload = EnvironmentAccount {
            platform: "os".to_string(),
            platform_url: "http://baidu.com".to_string(),
            platform_account: "lius".to_string(),
            platform_password: "lius".to_string(),
            environment_uuid: "3dcd8228-120b-4ae7-b8d7-da7e6628269e".to_string(),
            user_uuid: Some("3cfb0bc6-7b48-498a-935a-90ce561e40a5".to_string()),
            id: Some(1),
            ..Default::default()
        };
        let result = modify(&payload).await;
        println!("{:?}", result)
    }

    #[tokio::test]
    async fn test_delete() {
        crate::setup().await;

        let result = delete(1).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}
