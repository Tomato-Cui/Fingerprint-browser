use lp_cores::requests::{client, JsonRespnse};
use serde_json::json;

pub async fn query_by_id(id: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environment-accounts/query/id")?,
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
            client::Client::build_url("/environment-accounts/query")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn query_current_by_current_environment(
    environmnet_uuid: &str,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environmnet_uuid": environmnet_uuid,
        "page_num": page_num,
        "page_size": page_size,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environment-accounts/query/environment/uuid")?,
            &data,
        )
        .await?;

    Ok(json_response)
}

pub async fn create(
    account: lp_models::environment_account::EnvironmentAccount,
) -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environment-accounts")?,
            &account,
        )
        .await?;

    Ok(json_response)
}

pub async fn modify(
    account: lp_models::environment_account::EnvironmentAccount,
) -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .put(
            client::Client::build_url("/environment-accounts")?,
            &account,
        )
        .await?;

    Ok(json_response)
}

pub async fn delete(id: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
    });

    let json_response = client::REQUEST
        .delete(client::Client::build_url("/environment-accounts")?, &data)
        .await?;

    Ok(json_response)
}

pub async fn batch_delete(ids: Vec<u32>) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "ids": ids,
    });

    let json_response = client::REQUEST
        .delete(
            client::Client::build_url("/environment-accounts/batch")?,
            &data,
        )
        .await?;

    Ok(json_response)
}
