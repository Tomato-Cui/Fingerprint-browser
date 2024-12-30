use std::collections::HashMap;

use models::extension::{self, Extension};
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn insert(extension: extension::Extension) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = extension::Extension::insert(pool, extension).await?;
    Ok(ok)
}

pub async fn user_insert(user_uuid: &str, extnesion_uuid: &str) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = extension::Extension::user_insert(pool, user_uuid, extnesion_uuid).await?;
    Ok(ok)
}

pub async fn team_insert(team_id: &str, extension_uuid: &str) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = extension::Extension::team_insert(pool, team_id, extension_uuid).await?;
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

pub async fn query_environmnets_by_extension_uuid(
    extension_uuid: &str,
    page_num: u32,
    page_size: u32,
) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, extens) = extension::Extension::query_environmnet_uuid_by_extension_uuid(
        pool,
        extension_uuid,
        page_num,
        page_size,
    )
    .await?;

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

pub async fn user_toggle_extension(
    user_uuid: &str,
    extension_uuid: &str,
    open: bool,
) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok =
        extension::Extension::user_toggle_extension(pool, user_uuid, extension_uuid, open).await?;

    Ok(ok)
}

pub async fn environmnet_use_extension(
    extension_uuid: &str,
    environment_uuids: Vec<String>,
) -> Result<HashMap<String, bool>, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let mut res = HashMap::new();
    for environment_uuid in environment_uuids {
        let ok = extension::Extension::environmnet_use_extension(
            pool,
            extension_uuid,
            &environment_uuid,
        )
        .await?;

        res.insert(environment_uuid, ok);
    }

    Ok(res)
}

pub async fn environmnet_remove_extension(
    extension_uuid: &str,
    environment_uuid: &str,
) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok =
        extension::Extension::environmnet_remove_extension(pool, extension_uuid, &environment_uuid)
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

pub async fn remove_by_user_uuid(
    user_uuid: &str,
    extension_uuid: &str,
) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = extension::Extension::remove_by_user_uuid(pool, user_uuid, extension_uuid).await?;

    Ok(ok)
}

#[tokio::test]
async fn test_query_environmnets_by_extension_uuid() {
    crate::setup().await;
    let res = query_environmnets_by_extension_uuid("iginnfkhmmfhlkagcmpgofnjhanpmklb", 1, 100)
        .await
        .unwrap();

    println!("{:?}", res);
}
