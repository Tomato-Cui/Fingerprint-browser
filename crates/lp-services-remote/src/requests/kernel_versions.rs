use lp_cores::requests::{client, JsonRespnse};

pub async fn latest_kernel() -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .get(client::Client::build_url(
            "/kernel-versions/latest/version",
        )?)
        .await?;

    Ok(json_response)
}
