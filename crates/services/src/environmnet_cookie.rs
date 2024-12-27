use models::environmnet_cookie::EnvironmentCookie;

use crate::error::ServiceError;

pub async fn query_by_id(id: u32) -> Result<EnvironmentCookie, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let cookie = EnvironmentCookie::query_by_id(pool, id).await?;

    Ok(cookie)
}

pub async fn query(environmnet_uuid: &str) -> Result<Vec<EnvironmentCookie>, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let cookies = EnvironmentCookie::query_by_environment_uuid(pool, environmnet_uuid).await?;

    Ok(cookies)
}

pub async fn create(payload: &EnvironmentCookie) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentCookie::insert(pool, payload).await?;

    Ok(ok)
}

pub async fn modify(
    environmnet_uuid: &str,
    payload: &EnvironmentCookie,
) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentCookie::update(pool, environmnet_uuid, payload).await?;

    Ok(ok)
}

pub async fn delete(environmnet_uuid: &str) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentCookie::delete(pool, environmnet_uuid).await?;

    Ok(ok)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_by_id() {
        crate::setup().await;
        let result = query_by_id(1).await;

        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_query() {
        crate::setup().await;
        let result = query("e5368907-d858-47e4-bfee-eddabbd36a56").await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_create() {
        crate::setup().await;
        let environment_uuid = "e5368907-d858-47e4-bfee-eddabbd36a56";
        let payload = EnvironmentCookie {
            value: "abchdshfj".to_string(),
            environment_uuid: Some(environment_uuid.to_string()),
            ..Default::default()
        };
        let result = create(&payload).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_modify() {
        crate::setup().await;
        let payload = EnvironmentCookie {
            ..Default::default()
        };
        let result = modify("e5368907-d858-47e4-bfee-eddabbd36a56", &payload).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_delete() {
        crate::setup().await;
        let result = delete("e5368907-d858-47e4-bfee-eddabbd36a56").await;
        println!("{:?}", result);
    }
}
