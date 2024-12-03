use crate::{errors::ServerFetchError, Result};
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct JsonRespnse {
    pub code: Option<i32>,
    pub message: Option<String>,
    pub data: Option<Value>,
}

pub mod client {
    use crate::{
        auth::{clear_token, get_token},
        errors::ServerFetchError,
        Result,
    };
    use axum::http::HeaderValue;
    use once_cell::sync::Lazy;
    use reqwest::{
        header::{self, AUTHORIZATION},
        Response, StatusCode, Url,
    };
    use serde_json::Value;
    use std::{future::Future, pin::Pin, time::Duration};

    const SERVER_URL: &'static str = "http://192.168.6.116:8080";
    const TIMEOUT_DURATION_SECS: u64 = 10;

    type BeforeCallFunction = fn(
        rb: reqwest::RequestBuilder,
    ) -> Pin<
        Box<dyn Future<Output = std::result::Result<reqwest::RequestBuilder, reqwest::Error>>>,
    >;
    type AfterCallFunction = fn(
        response: reqwest::Response,
    ) -> Pin<
        Box<dyn Future<Output = std::result::Result<reqwest::Response, reqwest::Error>>>,
    >;

    pub struct Client {
        client: reqwest::Client,
        before: Vec<BeforeCallFunction>,
        after: Vec<AfterCallFunction>,
    }
    impl Client {
        pub fn new(timeout: u64) -> Client {
            Client {
                client: reqwest::Client::builder()
                    .timeout(Duration::from_secs(timeout))
                    .default_headers({
                        let headers = header::HeaderMap::new();
                        headers
                    })
                    .build()
                    .unwrap(),
                before: vec![],
                after: vec![],
            }
        }

        pub fn build_url(resource: &str) -> Result<Url> {
            Ok(Url::parse(SERVER_URL)
                .map_err(|_| ServerFetchError::ServerResponseParseFail)?
                .join(resource)
                .map_err(|_| ServerFetchError::ServerResponseParseFail)?)
        }

        pub async fn post(
            &self,
            url: reqwest::Url,
            json: &Value,
        ) -> core::result::Result<Response, reqwest::Error> {
            let mut request_builder = self.client.post(url);
            for call in &self.before {
                request_builder = call(request_builder).await?;
            }

            let mut response = request_builder.json(json).send().await?;
            for call in &self.after {
                response = call(response).await?;
            }
            Ok(response)
        }

        pub async fn get(
            &self,
            url: reqwest::Url,
        ) -> core::result::Result<Response, reqwest::Error> {
            let mut request_builder = self.client.post(url);
            for call in &self.before {
                request_builder = call(request_builder).await?;
            }
            let mut response = request_builder.send().await?;
            for call in &self.after {
                response = call(response).await?;
            }
            Ok(response)
        }

        pub fn before(&mut self, call: BeforeCallFunction) {
            self.before.push(call);
        }

        pub fn after(
            &mut self,
            call: AfterCallFunction, // call: fn(response: reqwest::Response) -> core::result::Result<Response, reqwest::Error>,
        ) {
            self.after.push(call);
        }
    }

    async fn before(
        rb: reqwest::RequestBuilder,
    ) -> core::result::Result<reqwest::RequestBuilder, reqwest::Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(token) = get_token().await {
            if let Ok(header_value) = HeaderValue::from_str(&format!("Bearer {}", token)) {
                headers.insert(AUTHORIZATION, header_value);
            }
        }
        Ok(rb.headers(headers))
    }
    async fn after(
        response: reqwest::Response,
    ) -> core::result::Result<reqwest::Response, reqwest::Error> {
        if let StatusCode::UNAUTHORIZED = response.status() {
            clear_token().await;
        }

        Ok(response)
    }

    pub static REQUEST: Lazy<Client> = Lazy::new(|| {
        let mut client = Client::new(TIMEOUT_DURATION_SECS);

        client.before(|rb| Box::pin(before(rb)));
        client.after(|response| Box::pin(after(response)));

        client
    });
}

pub mod auth {

    use crate::auth::{init_auth_state, set_token};

    use super::*;

    pub async fn login(username: &str, password: &str) -> Result<JsonRespnse> {
        let data = json!({
                "username": username,
                "password": password
        });

        let response = client::REQUEST
            .post(client::Client::build_url("/api/auth/login")?, &data)
            .await
            .map_err(|_| ServerFetchError::ServerRequestConnectFail)?;

        let json_response: JsonRespnse = response.json().await.map_err(|e| {
            if e.is_connect() {
                ServerFetchError::ServerRequestConnectFail
            } else {
                ServerFetchError::ServerResponseParseFail
            }
        })?;

        if let Some(Value::Object(obj)) = &json_response.data {
            if let Some(Value::String(token)) = obj.get("token") {
                init_auth_state().await;
                set_token(token).await;
            }
        }

        Ok(json_response)
    }

    pub async fn register(
        username: &str,
        password: &str,
        email: &str,
        captcha: &str,
    ) -> Result<Option<String>> {
        let data = json!({
            "account": {
                "email": email,
                "username": username,
                "password": password
            },
            "captcha": captcha
        });

        let response = client::REQUEST
            .post(client::Client::build_url("/api/auth/register")?, &data)
            .await
            .map_err(|_| ServerFetchError::ServerRequestConnectFail)?;

        let json_response: JsonRespnse = response
            .json()
            .await
            .map_err(|_| ServerFetchError::ServerResponseParseFail)?;

        Ok(json_response.message)
    }

    pub async fn send(email: &str) -> Result<Option<String>> {
        let data = json!({
                "email": email,
        });

        let response = client::REQUEST
            .post(client::Client::build_url("/api/auth/send")?, &data)
            .await
            .map_err(|_| ServerFetchError::ServerRequestConnectFail)?;

        let json_response: JsonRespnse = response
            .json()
            .await
            .map_err(|_| ServerFetchError::ServerResponseParseFail)?;

        Ok(json_response.message)
    }
}

pub mod group {
    // use super::*;
    // use crate::{errors::ServerFetchError, Result};
}
