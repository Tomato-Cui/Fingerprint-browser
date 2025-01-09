use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Debug, Serialize)]
pub struct JsonRespnse {
    pub code: Option<i32>,
    pub message: Option<String>,
    pub data: Option<Value>,
}

pub mod client {
    use axum::http::HeaderValue;
    use once_cell::sync::Lazy;
    use reqwest::{
        header::{self, AUTHORIZATION},
        StatusCode, Url,
    };
    use states::auth::{clear_token, get_token};
    use std::{fmt::Debug, future::Future, pin::Pin, time::Duration};

    use super::JsonRespnse;

    pub const TIMEOUT_DURATION_SECS: u64 = 60;

    type BeforeCallFunction = fn(
        rb: reqwest::RequestBuilder,
    ) -> Pin<
        Box<
            dyn Future<Output = std::result::Result<reqwest::RequestBuilder, reqwest::Error>>
                + Send,
        >,
    >;
    type AfterCallFunction = fn(
        response: reqwest::Response,
    ) -> Pin<
        Box<dyn Future<Output = std::result::Result<reqwest::Response, reqwest::Error>> + Send>,
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

        pub fn build_url(resource: &str) -> Result<Url, anyhow::Error> {
            let server_url = if let Some(app) = states::config::get_config() {
                &app.app.remote_url
            } else {
                "127.0.0.1:5678"
            };

            Ok(Url::parse(&server_url)
                .map_err(|e| anyhow::anyhow!("{:?}", e))?
                .join(&format!("/api/v1{}", resource))
                .map_err(|e| anyhow::anyhow!("{:?}", e))?)
        }

        pub async fn post<T>(
            &self,
            url: reqwest::Url,
            json: &T,
        ) -> core::result::Result<JsonRespnse, anyhow::Error>
        where
            T: serde::Serialize + Debug,
        {
            let mut request_builder = self.client.post(url);
            for call in &self.before {
                request_builder = call(request_builder).await?;
            }
            let mut response = request_builder.json(json).send().await?;

            for call in &self.after {
                response = call(response).await?;
            }
            let status = response.status();
            response
                .json()
                .await
                .map_err(|_| anyhow::anyhow!("StatusCode({:?})", status))
        }

        pub async fn get(
            &self,
            url: reqwest::Url,
        ) -> core::result::Result<JsonRespnse, anyhow::Error> {
            let mut request_builder = self.client.get(url);
            for call in &self.before {
                request_builder = call(request_builder).await?;
            }
            let mut response = request_builder.send().await?;
            for call in &self.after {
                response = call(response).await?;
            }

            let status = response.status();
            response
                .json()
                .await
                .map_err(|_| anyhow::anyhow!("访问错误: ({:?})", status))
        }

        pub async fn put<T>(
            &self,
            url: reqwest::Url,
            json: &T,
        ) -> core::result::Result<JsonRespnse, anyhow::Error>
        where
            T: serde::Serialize,
        {
            let mut request_builder = self.client.put(url);
            for call in &self.before {
                request_builder = call(request_builder).await?;
            }
            let mut response = request_builder.json(json).send().await?;
            for call in &self.after {
                response = call(response).await?;
            }
            let status = response.status();
            response
                .json()
                .await
                .map_err(|_| anyhow::anyhow!("访问错误: ({:?})", status))
        }

        pub async fn delete<T>(
            &self,
            url: reqwest::Url,
            json: &T,
        ) -> core::result::Result<JsonRespnse, anyhow::Error>
        where
            T: serde::Serialize,
        {
            let mut request_builder = self.client.delete(url);
            for call in &self.before {
                request_builder = call(request_builder).await?;
            }
            let mut response = request_builder.json(json).send().await?;
            for call in &self.after {
                response = call(response).await?;
            }
            let status = response.status();
            response
                .json()
                .await
                .map_err(|_| anyhow::anyhow!("访问错误: ({:?})", status))
        }

        pub fn before(&mut self, call: BeforeCallFunction) {
            self.before.push(call);
        }

        pub fn after(&mut self, call: AfterCallFunction) {
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

        // todo: 可以对错误进行处理

        Ok(response)
    }

    pub static REQUEST: Lazy<Client> = Lazy::new(|| {
        let mut client = Client::new(TIMEOUT_DURATION_SECS);

        client.before(|rb| Box::pin(before(rb)));
        client.after(|response| Box::pin(after(response)));
        client
    });
}
