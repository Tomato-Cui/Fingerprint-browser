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
        let result = query_by_id(1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_query() {
        let result = query("some_uuid").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create() {
        let payload = EnvironmentCookie {
            ..Default::default() // fill in the fields as necessary
        };
        let result = create(&payload).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_modify() {
        let payload = EnvironmentCookie {
            ..Default::default()
        };
        let result = modify("some_uuid", &payload).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete() {
        let result = delete("some_uuid").await;
        assert!(result.is_ok());
    }
}
