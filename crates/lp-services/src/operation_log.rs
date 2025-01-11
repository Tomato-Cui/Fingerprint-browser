use lp_models::operation_log::OperationLog;
use serde_json::{json, Value};

use crate::{error::ServiceError, team};

pub async fn record_operation_log(
    user_uuid: &str,
    resource_name: &str,
) -> Result<bool, ServiceError> {
    let allowed_operation = if let Some(value) =
        crate::allowed_operation::query_by_resource_name(resource_name).await?
    {
        value
    } else {
        return Ok(false);
    };

    let team_info = team::query_current_team_info(user_uuid)
        .await
        .map_err(|v| anyhow::anyhow!("查询用户所在团队失败: {:?}", v))?;

    let log = OperationLog {
        allowed_operation_id: allowed_operation.id.unwrap_or_default(),
        operation_status: Some(false),
        ..Default::default()
    };
    let ok = crate::operation_log::insert(user_uuid, team_info.id as u32, log).await?;

    Ok(ok)
}

pub async fn insert(
    user_uuid: &str,
    team_id: u32,
    operation_log: OperationLog,
) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;
    let ok = OperationLog::insert(pool, team_id, user_uuid, operation_log).await?;

    Ok(ok)
}

pub async fn query_by_id(id: u32) -> Result<OperationLog, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let operation_log = OperationLog::query_by_id(pool, id).await?;
    Ok(operation_log)
}

pub async fn query_by_user_uuid(
    user_uuid: &str,
    page_num: u32,
    page_size: u32,
) -> Result<Value, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let (total, data) =
        OperationLog::query_by_user_uuid(pool, user_uuid, page_num, page_size).await?;

    Ok(json! ({
        "total":total,
        "data":data,
    }))
}

pub async fn query_by_team_id(
    team_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<Value, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let (total, data) = OperationLog::query_by_team_id(pool, team_id, page_num, page_size).await?;

    Ok(json! ({
        "total":total,
        "data":data,
    }))
}

pub async fn update_status(id: u32, status: u8) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;
    let ok = OperationLog::update_status(pool, id, status).await?;
    Ok(ok)
}

pub async fn delete(id: u32) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let ok = OperationLog::delete(pool, id).await?;
    Ok(ok)
}
