use models::environment::Environment;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_id(id: i32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let environment = models::environment::Environment::query_by_id(pool, id).await?;

    Ok(json!({
        "data": environment,
    }))
}

pub async fn query(page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, environment) =
        models::environment::Environment::query_by_col(pool, "", "", page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": environment,
    }))
}

pub async fn query_by_col(
    page_num: u32,
    page_size: u32,
    col_key: &str,
    col_value: &str,
) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, environment) = models::environment::Environment::query_by_col(
        pool, col_key, col_value, page_num, page_size,
    )
    .await?;

    Ok(json!({
        "total": total,
        "data": environment,
    }))
}

pub async fn create(payload: &Environment) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = models::environment::Environment::insert(pool, payload).await?;

    Ok(ok)
}

pub async fn create_batch(payload: &Vec<Environment>) -> Result<Vec<Value>, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let mut response = vec![];
    for env_ in payload {
        let ok = models::environment::Environment::insert(pool, env_).await?;
        response.push(json!({
            "name": env_.name,
            "success": ok
        }));
    }

    Ok(response)
}

pub async fn modify(payload: &Environment) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = models::environment::Environment::update(pool, payload).await?;

    Ok(ok)
}

pub async fn move_to_group(env_id: i32, group_id: i32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let _ = models::group::Group::query_id(pool, group_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ServiceError::Error("指定分组不存在".to_string()),
            e => ServiceError::Error(e.to_string()),
        })?;
    let group_id = format!("{}", group_id);

    let ok = models::environment::Environment::update_by_col(pool, env_id, "group_id", &group_id)
        .await?;

    Ok(ok)
}

pub async fn batch_move_to_group(
    env_ids: Vec<i32>,
    group_id: i32,
) -> Result<Vec<Value>, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let _ = models::group::Group::query_id(pool, group_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ServiceError::Error("指定分组不存在".to_string()),
            e => ServiceError::Error(e.to_string()),
        })?;
    let group_id = format!("{}", group_id);

    let mut response = vec![];
    for env_id in env_ids {
        let ok =
            models::environment::Environment::update_by_col(pool, env_id, "group_id", &group_id)
                .await?;
        response.push(json!({
            "environment_id": env_id,
            "success": ok,
        }));
    }
    Ok(response)
}

pub async fn delete(env_id: i32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = models::environment::Environment::delete(pool, env_id).await?;

    Ok(ok)
}

pub async fn batch_delete(env_ids: Vec<i32>) -> Result<Vec<bool>, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let mut response = vec![];
    for env_id in env_ids {
        let ok = models::environment::Environment::delete(pool, env_id).await?;
        response.push(ok);
    }
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn test_query_by_id() {
        crate::setup().await;

        let ok = super::query_by_id(1).await.unwrap();
        println!("{}", ok);
    }

    #[tokio::test]
    pub async fn test_query() {
        crate::setup().await;

        let ok = super::query(1, 10).await.unwrap();
        println!("{}", ok);
    }

    #[tokio::test]
    pub async fn test_query_by_col() {
        crate::setup().await;

        let value = super::query_by_col(1, 10, "name", "").await.unwrap();
        println!("{}", value);
    }

    #[tokio::test]
    pub async fn test_create_batch() {
        crate::setup().await;

        let value = super::create_batch(&vec![
            Environment {
                name: "test".to_string(),
                ..Default::default()
            },
            Environment {
                name: "test2".to_string(),
                ..Default::default()
            },
        ])
        .await
        .unwrap();

        println!("{:?}", value);
    }

    #[tokio::test]
    pub async fn test_modify() {
        crate::setup().await;

        let ok = super::modify(&Environment {
            id: Some(1),
            name: "abc".to_string(),
            ..Default::default()
        })
        .await
        .unwrap();

        println!("{}", ok);
    }

    #[tokio::test]
    pub async fn test_move_to_group() {
        crate::setup().await;

        let ok = super::move_to_group(1, 10).await;
        println!("{:?}", ok);
    }

    #[tokio::test]
    pub async fn test_batch_move_to_group() {
        crate::setup().await;

        let ok = super::batch_move_to_group(vec![1, 2, 3], 10).await;
        println!("{:?}", ok);
    }

    #[tokio::test]
    pub async fn test_delete() {
        crate::setup().await;

        let ok = super::delete(1).await;
        println!("{:?}", ok);
    }
}
