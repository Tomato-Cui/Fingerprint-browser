
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query(user_id: u32, page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, environment) =
        models::environment::Environment::query_delete(pool, user_id, page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": environment,
    }))
}

pub async fn recover(user_id: u32, id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let ok = models::environment::Environment::recover(pool, user_id, id).await?;
    Ok(ok)
}

pub async fn recovers(user_id: u32, ids: Vec<u32>) -> Result<Vec<Value>, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let mut response = vec![];
    for id in ids {
        let ok = models::environment::Environment::recover(pool, user_id, id).await?;
        response.push(json!({
            "id": id,
            "success": ok
        }));
    }
    Ok(response)
}

pub async fn recover_all(user_id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = models::environment::Environment::recover_all(pool, user_id).await?;
    Ok(ok)
}

pub async fn delete_again(user_id: u32, env_id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let ok = models::environment::Environment::delete_again(pool, user_id, env_id).await?;

    Ok(ok)
}

pub async fn batch_delete_again(user_id: u32, env_ids: Vec<u32>) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let ok = models::environment::Environment::batch_delete_again(pool, user_id, env_ids).await?;

    Ok(ok)
}

pub async fn clean(user_id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = models::environment::Environment::clean(pool, user_id).await?;

    Ok(ok)
}
