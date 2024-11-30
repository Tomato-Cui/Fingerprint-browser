use std::{collections::HashMap, time::Duration};

use crate::{errors::ApplicationServerError, Result};
use once_cell::sync::Lazy;
use reqwest::{header, Client, Url};
use serde::Deserialize;

const SERVER_URL: &'static str = "http://192.168.6.116:8080";
const TIMEOUT_DURATION_SECS: u64 = 10;

#[derive(Deserialize, Debug)]
pub struct JsonRespnse<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
}

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

    let json_response: JsonRespnse<String> = response.json().await?;
    println!("resutl:{:?}", json_response);

    Ok(())
}

// pub async fn register(password: &str, password: &str) -> Result<()> {
//     let mut map = HashMap::new();
//     map.insert("username", username);
//     map.insert("password", password);

//     let response = REQUEST
//         .post(build_url("/api/auth/login")?)
//         .json(&map)
//         .send()
//         .await?;

//     let json_response: JsonRespnse<String> = response.json().await?;
//     println!("resutl:{:?}", json_response);

//     Ok(())
// }
