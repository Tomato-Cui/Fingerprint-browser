use models::extension::{self, Extension};
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn user_insert(
    user_uuid: &str,
    extension: extension::Extension,
) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok =
        extension::Extension::insert(pool, "user_extension_relation", user_uuid, extension).await?;
    Ok(ok)
}

pub async fn team_insert(
    team_id: &str,
    extension: extension::Extension,
) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok =
        extension::Extension::insert(pool, "team_extension_relation", team_id, extension).await?;
    Ok(ok)
}

pub async fn query_by_team_id(
    team_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, extens) =
        extension::Extension::query_by_team_id(pool, team_id, page_num, page_size).await?;

    Ok(json!({
        "total":total,
        "data": extens,
    }))
}

pub async fn query_by_user_uuid(
    user_uuid: &str,
    page_num: u32,
    page_size: u32,
) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, extens) =
        extension::Extension::query_by_user_uuid(pool, user_uuid, page_num, page_size).await?;

    Ok(json!({
        "total":total,
        "data": extens,
    }))
}

pub async fn query_by_environmnet_uuid(
    environmnet_uuid: &str,
    page_num: u32,
    page_size: u32,
) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, extens) = extension::Extension::query_by_environmnet_uuid(
        pool,
        environmnet_uuid,
        page_num,
        page_size,
    )
    .await?;

    Ok(json!({
        "total":total,
        "data": extens,
    }))
}

pub async fn query(page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, extens) = extension::Extension::query(pool, page_num, page_size).await?;

    Ok(json!({
        "total":total,
        "data": extens,
    }))
}

pub async fn environmnet_use_extension(
    extension_uuid: &str,
    environment_uuid: &str,
) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok =
        extension::Extension::environmnet_use_extension(pool, extension_uuid, environment_uuid)
            .await?;
    Ok(ok)
}

pub async fn update(extension_uuid: &str, extension: Extension) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = extension::Extension::update(pool, extension_uuid, extension).await?;

    Ok(ok)
}

pub async fn delete(extension_uuid: &str) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = extension::Extension::delete(pool, extension_uuid).await?;

    Ok(ok)
}
