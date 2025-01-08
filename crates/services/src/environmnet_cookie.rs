use models::environmnet_cookie::EnvironmentCookie;

use crate::error::ServiceError;

pub async fn query_by_id(id: u32) -> Result<EnvironmentCookie, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let cookie = EnvironmentCookie::query_by_id(pool, id).await?;

    Ok(cookie)
}

pub async fn query(environment_uuid: &str) -> Result<Vec<EnvironmentCookie>, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let cookies = EnvironmentCookie::query_by_environment_uuid(pool, environment_uuid).await?;

    Ok(cookies)
}

pub async fn create(payload: &EnvironmentCookie) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentCookie::insert(pool, payload).await?;

    Ok(ok)
}

pub async fn modify(
    environment_uuid: &str,
    payload: &EnvironmentCookie,
) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentCookie::update(pool, environment_uuid, payload).await?;

    Ok(ok)
}

pub async fn delete(environmnet_uuid: &str) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentCookie::delete(pool, environmnet_uuid).await?;

    Ok(ok)
}
