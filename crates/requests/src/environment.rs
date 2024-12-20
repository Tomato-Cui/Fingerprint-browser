use cores::request::{client, JsonRespnse};
use serde_json::{json, Value};

#[allow(dependency_on_unit_never_type_fallback)]
pub async fn get_environment_by_group_id(
    group_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<Value, anyhow::Error> {
    let response = client::REQUEST
        .get(client::Client::build_url(&format!(
            "/api/environments/GetEnvironmentGroup/{}?page={}&limit={}",
            group_id, page_num, page_size
        ))?)
        .await
        .map_err(|e| anyhow::anyhow!("访问失败({})", e))?;

    let json_response: JsonRespnse = response
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("访问失败({})", e))?;

    if let Some(d1) = json_response.data {
        if let Some(d2) = d1.get("data") {
            let total = d1
                .get("total_count")
                .unwrap_or(&serde_json::Value::Number(0.into()))
                .as_i64()
                .unwrap_or(0) as i64;

            return Ok(json!({
                "total": total,
                "data":serde_json::from_value(d2.clone())?,
            }));
        }
    }
    return Err(anyhow::anyhow!("访问失败({:?})", json_response.message));
}

pub async fn get_environment_by_id(id: i32) -> Result<Value, anyhow::Error> {
    let response = client::REQUEST
        .get(client::Client::build_url(&format!(
            "/api/environments/GetEnvironment/{}",
            id
        ))?)
        .await
        .map_err(|e| anyhow::anyhow!("访问失败({})", e))?;

    let json_response: JsonRespnse = response
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("访问失败({})", e))?;

    if let Some(data) = json_response.data {
        return Ok(serde_json::from_value(data.clone())?);
    }

    return Err(anyhow::anyhow!("访问失败({:?})", json_response.message));
}

#[allow(dependency_on_unit_never_type_fallback)]
pub async fn get_environment_list(page_num: u32, page_size: u32) -> Result<Value, anyhow::Error> {
    let response = client::REQUEST
        .get(client::Client::build_url(&format!(
            "/api/environments/getbypage?page={}&limit={}",
            page_num, page_size
        ))?)
        .await
        .map_err(|e| anyhow::anyhow!("访问失败({})", e))?;

    let json_response: JsonRespnse = response
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("访问失败({})", e))?;

    if let Some(d1) = json_response.data {
        if let Some(d2) = d1.get("data") {
            let total = d1
                .get("total_count")
                .unwrap_or(&serde_json::Value::Number(0.into()))
                .as_i64()
                .unwrap_or(0) as i64;

            return Ok(json!({
                "total": total,
                "data":serde_json::from_value(d2.clone())?,
            }));
        }
    }

    return Err(anyhow::anyhow!("访问失败({:?})", json_response.message));
}

pub async fn create_environment(
    payload: models::environment::Environment,
) -> Result<(), anyhow::Error> {
    let response = client::REQUEST
        .post(
            client::Client::build_url("/api/environments/CreateEnvironment")?,
            &payload,
        )
        .await
        .map_err(|e| anyhow::anyhow!("访问失败({})", e))?;

    let json_response: JsonRespnse = response
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("访问失败({})", e))?;

    if json_response.code.unwrap_or_default() == 1 {
        return Ok(());
    }

    return Err(anyhow::anyhow!("访问失败({:?})", json_response.message));
}

pub async fn update_environment(
    id: i32,
    payload: models::environment::Environment,
) -> Result<bool, anyhow::Error> {
    if id == 0 {
        return Ok(false);
    }

    let response = client::REQUEST
        .put(
            client::Client::build_url(&format!("/api/environments/{}", id,))?,
            &payload,
        )
        .await
        .map_err(|e| anyhow::anyhow!("访问失败({})", e))?;

    let json_response: JsonRespnse = response
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("访问失败({})", e))?;

    if json_response.code.unwrap_or_default() == 1 {
        return Ok(true);
    }

    Ok(false)
}

pub async fn delete_environment(id: i32) -> Result<bool, anyhow::Error> {
    if id == 0 {
        return Ok(false);
    }
    let response = client::REQUEST
        .delete(client::Client::build_url(&format!(
            "/api/environments/{}",
            id,
        ))?)
        .await
        .map_err(|e| anyhow::anyhow!("访问失败({})", e))?;

    let json_response: JsonRespnse = response
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("访问失败({})", e))?;

    if json_response.code.unwrap_or_default() == 1 {
        return Ok(true);
    }

    Ok(false)
}

pub async fn update_environment_by_json(
    id: i32,
    payload: serde_json::Value,
) -> Result<bool, anyhow::Error> {
    let response = client::REQUEST
        .put(
            client::Client::build_url(&format!("/api/environments/{}", id,))?,
            &payload,
        )
        .await
        .map_err(|e| anyhow::anyhow!("访问失败({})", e))?;

    let json_response: JsonRespnse = response
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("访问失败({})", e))?;

    if json_response.code.unwrap_or_default() == 1 {
        return Ok(true);
    }

    Ok(false)
}
