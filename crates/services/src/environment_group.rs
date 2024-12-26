use models::environment_group::EnvironmentGroup;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_id(id: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let group = EnvironmentGroup::query_by_id(pool, id).await?;

    Ok(json!({
        "data": group,
    }))
}

pub async fn query(user_uuid: &str, page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, groups) =
        EnvironmentGroup::query_groups_by_user_uuid(pool, user_uuid, page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": groups,
    }))
}

pub async fn create(payload: &EnvironmentGroup) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentGroup::insert(pool, payload).await?;

    Ok(ok)
}

pub async fn modify(id: u32, payload: &EnvironmentGroup) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentGroup::update(pool, id, payload).await?;

    Ok(ok)
}

pub async fn delete(group_id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = EnvironmentGroup::delete(pool, group_id).await?;

    Ok(ok)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_by_id() {
        let result = query_by_id(1).await;
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value.get("data").is_some());
    }

    #[tokio::test]
    async fn test_query() {
        crate::setup().await;

        let result = query("e972d1df-ff52-447e-9e30-8c5af698f5e8", 0, 10).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_create() {
        crate::setup().await;
        let payload = EnvironmentGroup {
            user_uuid: "e972d1df-ff52-447e-9e30-8c5af698f5e8".to_string(),
            name: "bac".to_string(),
            description: Some("bac".to_string()),
            ..Default::default()
        };

        let result = create(&payload).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_modify() {
        let payload = EnvironmentGroup {
            ..Default::default()
        };
        let result = modify(1, &payload).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_delete() {
        let result = delete(1).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}
