use models::team::Team;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_id(user_id: u32, id: u32) -> Result<Team, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let team = Team::query_id(pool, user_id, id).await?;

    Ok(team)
}

pub async fn query(user_id: u32, page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, team) = Team::query(pool, user_id, page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": team,
    }))
}

pub async fn create(user_id: u32, payload: &Team) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Team::insert(pool, user_id, payload).await?;

    Ok(ok)
}

pub async fn modify(user_id: u32, payload: &Team) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Team::update(pool, user_id, payload).await?;

    Ok(ok)
}

pub async fn delete(user_id: u32, id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Team::delete(pool, user_id, id).await?;

    Ok(ok)
}
