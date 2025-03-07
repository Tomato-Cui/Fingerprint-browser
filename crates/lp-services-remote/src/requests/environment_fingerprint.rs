use lp_cores::requests::{client, JsonRespnse};
use serde_json::json;

pub async fn query_by_id(id: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environment-fingerprints/query/id")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn query(page_num: u32, page_size: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "page_num": page_num,
        "page_size": page_size,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environment-fingerprints/query")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn create(
    fingerprint: lp_models::environment_fingerprint::EnvironmentFingerprint,
) -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environment-fingerprints")?,
            &fingerprint,
        )
        .await?;

    Ok(json_response)
}

pub async fn modify(
    fingerprint: lp_models::environment_fingerprint::EnvironmentFingerprint,
) -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .put(
            client::Client::build_url("/environment-fingerprints")?,
            &fingerprint,
        )
        .await?;

    Ok(json_response)
}

pub async fn modify_ua(id: i32, ua: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
        "ua": ua,
    });
    let json_response = client::REQUEST
        .put(
            client::Client::build_url("/environment-fingerprints/modify/ua")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn modify_by_colname(
    id: i32,
    col_name: &str,
    col_value: &str,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
        "col_name": col_name,
        "col_value": col_value,
    });
    let json_response = client::REQUEST
        .put(
            client::Client::build_url("/environment-fingerprints/modify/col/value")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn delete(id: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
    });

    let json_response = client::REQUEST
        .delete(
            client::Client::build_url("/environment-fingerprints")?,
            &data,
        )
        .await?;

    Ok(json_response)
}
