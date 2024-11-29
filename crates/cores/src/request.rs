use std::{collections::HashMap, time::Duration};

use crate::{errors::ApplicationServerError, Result};
use once_cell::sync::Lazy;
use reqwest::{header, Client, Url};

const SERVER_URL: &'static str = "http://192.168.6.116:8080";
const TIMEOUT_DURATION_SECS: u64 = 10;

static REQUEST: Lazy<Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(TIMEOUT_DURATION_SECS))
        .default_headers({
            let headers = header::HeaderMap::new();
            headers
        })
        .build()
        .unwrap()
});

fn build_url(resource: &str) -> Result<Url> {
    Ok(Url::parse(SERVER_URL)
        .map_err(|_| ApplicationServerError::UrlParseFail)?
        .join(resource)
        .map_err(|_| ApplicationServerError::UrlParseFail)?)
}

pub async fn login(username: &str, password: &str) -> Result<()> {
    let mut map = HashMap::new();
    map.insert("username", username);
    map.insert("password", password);

    let response = REQUEST
        .post(build_url("/api/auth/login")?)
        .json(&map)
        .send()
        .await?;

    let result = response.text().await?;
    println!("resutl:{:?}", result);

    Ok(())
}
