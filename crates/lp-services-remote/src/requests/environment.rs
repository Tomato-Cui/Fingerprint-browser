use lp_cores::requests::{client, JsonRespnse};
use serde_json::json;

pub async fn query_by_uuid(environment_uuid: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuid": environment_uuid,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environments/query/uuid")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

#[tokio::test]
async fn test_query_by_uuid() {
    crate::setup().await;
    let _t = crate::requests::user::login("1", "1").await;

    let environment_uuid = "d34f3e32-f0c1-44a2-a4c0-e43e2c7b0033";
    match query_by_uuid(&environment_uuid).await {
        Ok(json_response) => {
            if let Some(data) = json_response.data {
                let data = serde_json::from_value::<
                    lp_models::dto::environment_info::EnvironmentDetailWithResponse,
                >(data);
                eprintln!("{:?}", data);
            }
        }
        Err(_) => {}
    }
}

pub async fn query(page_num: u32, page_size: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "page_num": page_num,
        "page_size": page_size,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/environments/query")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn query_by_group(
    group_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "group_id": group_id,
        "page_num": page_num,
        "page_size": page_size,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environments/query/group")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn query_by_team(
    team_id: u32,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "team_id": team_id,
        "page_num": page_num,
        "page_size": page_size,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environments/query/team")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn query_by_extension(
    extension_uuid: &str,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "extension_uuid": extension_uuid,
        "page_num": page_num,
        "page_size": page_size,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environments/query/extension")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn simple_create(
    browser_type: &str,
    os_type: &str,
    numbers: u32,
    group_id: Option<u32>,
    use_encrypt: bool,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "browser_type": browser_type,
        "os_type": os_type,
        "numbers": numbers,
        "group_id": group_id,
        "use_encrypt": use_encrypt,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environments/simple-create")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn advanced_create(
    numbers: u32,
    use_encrypt: bool,
    environment: lp_models::dto::environment_info::EnvironmentDetailWithAdvanceCreateRequest,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "numbers": numbers,
        "use_encrypt": use_encrypt,
        "environment": environment,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environments/advanced-create")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn modify_info(
    environment_info: lp_models::environment::EnvironmentInfo,
) -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .put(
            client::Client::build_url(&format!("/environments"))?,
            &environment_info,
        )
        .await?;

    Ok(json_response)
}

pub async fn modify_proxy(
    environment_uuid: &str,
    proxy: lp_models::environment_proxies::Proxy,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuid": environment_uuid,
        "porxy": proxy,
    });

    let json_response = client::REQUEST
        .put(client::Client::build_url("/environments/proxy")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn advanced_modify(
    uuid: &str,
    environment: lp_models::dto::environment_info::EnvironmentDetailWithAdvanceCreateRequest,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "uuid": uuid,
        "environment": environment,
    });

    let json_response = client::REQUEST
        .put(
            client::Client::build_url("/environments/modify/advanced")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn modify_basic_info(
    environment: lp_models::environment::Environment,
) -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .put(
            client::Client::build_url("/environments/basic")?,
            &environment,
        )
        .await?;

    Ok(json_response)
}

pub async fn move_to_group(
    environment_uuid: &str,
    group_id: u32,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuid": environment_uuid,
        "group_id": group_id,
    });

    let json_response = client::REQUEST
        .put(
            client::Client::build_url("/environments/move-to-group")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn batch_move_to_group(
    environment_ids: Vec<String>,
    group_id: u32,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_ids": environment_ids,
        "group_id": group_id,
    });

    let json_response = client::REQUEST
        .put(
            client::Client::build_url("/environments/batch/move-to-group")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn batch_move_to_tag(
    environment_ids: Vec<String>,
    tag_id: u32,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_ids": environment_ids,
        "tag_id": tag_id,
    });

    let json_response = client::REQUEST
        .put(
            client::Client::build_url("/environments/batch/move-to-tag")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn delete(environment_uuid: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuid": environment_uuid,
    });

    let json_response = client::REQUEST
        .delete(client::Client::build_url("/environments")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn batch_delete(environment_uuids: Vec<String>) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuids": environment_uuids,
    });

    let json_response = client::REQUEST
        .delete(client::Client::build_url("/environments/batch")?, &data)
        .await?;

    Ok(json_response)
}
