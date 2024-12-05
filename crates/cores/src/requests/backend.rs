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
        errors::ApplicationServerError,
        Result,
    };
    use axum::http::HeaderValue;
    use once_cell::sync::Lazy;
    use reqwest::{
        header::{self, AUTHORIZATION},
        Response, StatusCode, Url,
    };
    use std::{future::Future, pin::Pin, time::Duration};

    const SERVER_URL: &'static str = "http://192.168.6.116:8080";
    const TIMEOUT_DURATION_SECS: u64 = 10;

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

        pub fn build_url(resource: &str) -> Result<Url> {
            Ok(Url::parse(SERVER_URL)
                .map_err(|e| ApplicationServerError::Error(anyhow::anyhow!("{:?}", e)))?
                .join(resource)
                .map_err(|e| ApplicationServerError::Error(anyhow::anyhow!("{:?}", e)))?)
        }

        pub async fn post<T>(
            &self,
            url: reqwest::Url,
            json: &T,
        ) -> core::result::Result<Response, reqwest::Error>
        where
            T: serde::Serialize,
        {
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
            let mut request_builder = self.client.get(url);
            for call in &self.before {
                request_builder = call(request_builder).await?;
            }
            let mut response = request_builder.send().await?;
            for call in &self.after {
                response = call(response).await?;
            }
            Ok(response)
        }

        pub async fn put<T>(
            &self,
            url: reqwest::Url,
            json: &T,
        ) -> core::result::Result<Response, reqwest::Error>
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
            Ok(response)
        }

        pub async fn delete(
            &self,
            url: reqwest::Url,
        ) -> core::result::Result<Response, reqwest::Error> {
            let mut request_builder = self.client.delete(url);
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
            if let Ok(header_value) = HeaderValue::from_str(&format!("{}", token)) {
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
    use crate::{
        auth::{clear_token, init_auth_state, set_token},
        utils::response::AppResponse,
    };

    use super::*;

    pub async fn logout() -> Result<AppResponse<()>> {
        clear_token().await;
        Ok(AppResponse::success(
            Some("logout success.".to_string()),
            Some(()),
        ))
    }

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
                ServerFetchError::ServerResponseParseFail(e)
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
            .map_err(|e| ServerFetchError::ServerResponseParseFail(e))?;

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
            .map_err(|e| ServerFetchError::ServerResponseParseFail(e))?;

        Ok(json_response.message)
    }
}

pub mod environment {
    use super::{client, JsonRespnse};
    use crate::errors::ServerFetchError;
    use crate::Result;
    use crate::{apis::PageParam, models::enviroment::Environment};

    pub async fn get_environment_by_group_id(
        id: i32,
        payload: &PageParam,
    ) -> Result<(i64, Vec<Environment>)> {
        let response = client::REQUEST
            .get(client::Client::build_url(&format!(
                "/api/environments/GetEnvironmentGroup/{}?page={}&limit={}",
                id,
                payload.page_num.unwrap_or_default(),
                payload.page_size.unwrap_or_default(),
            ))?)
            .await
            .map_err(|_| ServerFetchError::ServerRequestConnectFail)?;

        let json_response: JsonRespnse = response
            .json()
            .await
            .map_err(|e| ServerFetchError::ServerResponseParseFail(e))?;

        if let Some(d1) = json_response.data {
            if let Some(d2) = d1.get("data") {
                let total = d1
                    .get("total_count")
                    .unwrap_or(&serde_json::Value::Number(0.into()))
                    .as_i64()
                    .unwrap_or(0) as i64;

                return Ok((total, serde_json::from_value(d2.clone())?));
            }
        }

        Ok((0, vec![]))
    }

    pub async fn get_environment_by_id(id: i32) -> Result<Option<Environment>> {
        let response = client::REQUEST
            .get(client::Client::build_url(&format!(
                "/api/environments/GetEnvironment/{}",
                id
            ))?)
            .await
            .map_err(|_| ServerFetchError::ServerRequestConnectFail)?;

        let json_response: JsonRespnse = response
            .json()
            .await
            .map_err(|e| ServerFetchError::ServerResponseParseFail(e))?;

        if let Some(data) = json_response.data {
            return Ok(Some(serde_json::from_value(data.clone())?));
        }

        Ok(None)
    }

    pub async fn get_environment_list(payload: &PageParam) -> Result<(i64, Vec<Environment>)> {
        let response = client::REQUEST
            .get(client::Client::build_url(&format!(
                "/api/environments/getbypage?page={}&limit={}",
                payload.page_num.unwrap_or_default(),
                payload.page_size.unwrap_or_default()
            ))?)
            .await
            .map_err(|_| ServerFetchError::ServerRequestConnectFail)?;

        let json_response: JsonRespnse = response
            .json()
            .await
            .map_err(|e| ServerFetchError::ServerResponseParseFail(e))?;

        if let Some(d1) = json_response.data {
            if let Some(d2) = d1.get("data") {
                let total = d1
                    .get("total_count")
                    .unwrap_or(&serde_json::Value::Number(0.into()))
                    .as_i64()
                    .unwrap_or(0) as i64;

                return Ok((total, serde_json::from_value(d2.clone())?));
            }
        }
        Ok((0, vec![]))
    }

    pub async fn create_environment(payload: Environment) -> Result<bool> {
        let response = client::REQUEST
            .post(
                client::Client::build_url("/api/environments/CreateEnvironment")?,
                &payload,
            )
            .await
            .map_err(|_| ServerFetchError::ServerRequestConnectFail)?;

        let json_response: JsonRespnse = response
            .json()
            .await
            .map_err(|e| ServerFetchError::ServerResponseParseFail(e))?;

        if json_response.code.unwrap_or_default() == 1 {
            return Ok(true);
        }

        Ok(false)
    }

    pub async fn update_environment(id: i32, payload: Environment) -> Result<bool> {
        if id == 0 {
            return Ok(false);
        }

        let response = client::REQUEST
            .put(
                client::Client::build_url(&format!("/api/environments/{}", id,))?,
                &payload,
            )
            .await
            .map_err(|_| ServerFetchError::ServerRequestConnectFail)?;

        let json_response: JsonRespnse = response
            .json()
            .await
            .map_err(|e| ServerFetchError::ServerResponseParseFail(e))?;

        if json_response.code.unwrap_or_default() == 1 {
            return Ok(true);
        }

        Ok(false)
    }

    pub async fn delete_environment(id: i32) -> Result<bool> {
        if id == 0 {
            return Ok(false);
        }
        let response = client::REQUEST
            .delete(client::Client::build_url(&format!(
                "/api/environments/{}",
                id,
            ))?)
            .await
            .map_err(|_| ServerFetchError::ServerRequestConnectFail)?;

        let json_response: JsonRespnse = response
            .json()
            .await
            .map_err(|e| ServerFetchError::ServerResponseParseFail(e))?;

        if json_response.code.unwrap_or_default() == 1 {
            return Ok(true);
        }

        Ok(false)
    }

    pub async fn update_environment_by_json(id: i32, payload: serde_json::Value) -> Result<bool> {
        let response = client::REQUEST
            .put(
                client::Client::build_url(&format!("/api/environments/{}", id,))?,
                &payload,
            )
            .await
            .map_err(|_| ServerFetchError::ServerRequestConnectFail)?;

        let json_response: JsonRespnse = response
            .json()
            .await
            .map_err(|e| ServerFetchError::ServerResponseParseFail(e))?;

        if json_response.code.unwrap_or_default() == 1 {
            return Ok(true);
        }

        Ok(false)
    }
}

pub mod group {

    use serde_json::json;

    use super::{client, JsonRespnse};
    use crate::apis::PageParam;
    use crate::errors::ServerFetchError;
    use crate::models::group::Group;
    use crate::Result;

    pub async fn get_group_list(payload: &PageParam) -> Result<(i64, Vec<Group>)> {
        let response = client::REQUEST
            .get(client::Client::build_url(&format!(
                "/api/environments/GetEnvironmentGroupBypage?page={}&limit={}",
                payload.page_num.unwrap_or_default(),
                payload.page_size.unwrap_or_default()
            ))?)
            .await
            .map_err(|_| ServerFetchError::ServerRequestConnectFail)?;

        let json_response: JsonRespnse = response
            .json()
            .await
            .map_err(|e| ServerFetchError::ServerResponseParseFail(e))?;

        if let Some(d1) = json_response.data {
            if let Some(d2) = d1.get("data") {
                let total = d1
                    .get("total_count")
                    .unwrap_or(&serde_json::Value::Number(0.into()))
                    .as_i64()
                    .unwrap_or(0) as i64;

                return Ok((total, serde_json::from_value(d2.clone())?));
            }
        }
        Ok((0, vec![]))
    }

    pub async fn update_group(id: i32, name: &str, description: &str) -> Result<bool> {
        let payload = json!({
            "name":name,
            "description":description,
        });
        let response = client::REQUEST
            .put(
                client::Client::build_url(&format!("/api/environments/groups/{}", id))?,
                &payload,
            )
            .await
            .map_err(|_| ServerFetchError::ServerRequestConnectFail)?;

        let json_response: JsonRespnse = response
            .json()
            .await
            .map_err(|e| ServerFetchError::ServerResponseParseFail(e))?;

        if json_response.code.unwrap_or_default() == 1 {
            return Ok(true);
        }

        Ok(false)
    }

    pub async fn delete_group(id: i32) -> Result<bool> {
        let response = client::REQUEST
            .delete(client::Client::build_url(&format!(
                "/api/environments/groups/{}",
                id
            ))?)
            .await
            .map_err(|_| ServerFetchError::ServerRequestConnectFail)?;

        let json_response: JsonRespnse = response
            .json()
            .await
            .map_err(|e| ServerFetchError::ServerResponseParseFail(e))?;

        if json_response.code.unwrap_or_default() == 1 {
            return Ok(true);
        }

        Ok(false)
    }

    pub async fn create_group(name: &str, description: &str) -> Result<bool> {
        let payload = json!({
            "name":name,
            "description":description,
        });

        let response = client::REQUEST
            .post(
                client::Client::build_url(&format!("/api/environments/groups/create",))?,
                &payload,
            )
            .await
            .map_err(|_| ServerFetchError::ServerRequestConnectFail)?;

        let json_response: JsonRespnse = response
            .json()
            .await
            .map_err(|e| ServerFetchError::ServerResponseParseFail(e))?;

        if json_response.code.unwrap_or_default() == 1 {
            return Ok(true);
        }

        Ok(false)
    }
}
