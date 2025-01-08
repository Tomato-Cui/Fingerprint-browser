use models::allowed_operation::AllowedOperation;

use crate::error::ServiceError;

pub async fn insert(allowed_operation: AllowedOperation) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let ok = AllowedOperation::insert(pool, allowed_operation).await?;

    Ok(ok)
}

pub async fn query_by_id(id: u32) -> Result<AllowedOperation, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let allowed_operation = AllowedOperation::query_by_id(pool, id).await?;
    Ok(allowed_operation)
}

pub async fn query_by_resource_name(
    resource_name: &str,
) -> Result<Option<AllowedOperation>, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let allowed_operation = AllowedOperation::query_by_resource_name(pool, resource_name).await?;

    Ok(allowed_operation)
}

pub async fn update(id: u32, allowed_operation: AllowedOperation) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let ok = AllowedOperation::update(pool, id, allowed_operation).await?;
    Ok(ok)
}

pub async fn delete(id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = AllowedOperation::delete(pool, id).await?;
    Ok(ok)
}
