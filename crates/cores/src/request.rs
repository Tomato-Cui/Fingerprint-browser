use std::{collections::HashMap, time::Duration};

use crate::{errors::ApplicationServerError, Result};
use once_cell::sync::Lazy;
use reqwest::{header, Client, Url};
use serde::Deserialize;

const SERVER_URL: &'static str = "http://192.168.6.116:8080";
const TIMEOUT_DURATION_SECS: u64 = 10;

#[derive(Deserialize, Debug)]
pub struct JsonRespnse<T> {
    pub code: Option<i32>,
    pub message: Option<String>,
    pub data: Option<T>,
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
pub mod auth {
    use serde_json::json;

    use super::*;

    pub async fn login(username: &str, password: &str) -> Result<()> {
        let data = json!({
                "username": username,
                "password": password
        });

        let response = REQUEST
            .post(build_url("/api/auth/login")?)
            .json(&data)
            .send()
            .await?;

        let json_response: JsonRespnse<String> = response.json().await?;
        println!("resutl:{:?}", json_response);

        Ok(())
    }

    pub async fn register(
        username: &str,
        password: &str,
        email: &str,
        captcha: &str,
    ) -> Result<()> {
        let data = json!({
            "account": {
                "email": email,
                "username": username,
                "password": password
            },
            "captcha": captcha
        });

        let response = REQUEST
            .post(build_url("/api/auth/register")?)
            .json(&data)
            .send()
            .await?;

        let json_response: JsonRespnse<String> = response.json().await?;
        println!("resutl:{:?}", json_response);

        Ok(())
    }

    pub async fn send(email: &str) -> Result<()> {
        // pub async fn register(password: &str, password: &str) -> Result<()> {
        let data = json!({
                "email": email,
        });

        let response = REQUEST
            .post(build_url("/api/auth/send")?)
            .json(&data)
            .send()
            .await?;

        let json_response: JsonRespnse<String> = response.json().await?;
        println!("resutl:{:?}", json_response);

        Ok(())
    }
}
