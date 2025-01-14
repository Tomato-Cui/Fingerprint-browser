use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;
use tokio::{
    fs,
    io::{stdout, AsyncWriteExt},
};

#[derive(Deserialize, Debug, Serialize)]
pub struct JsonRespnse {
    pub code: Option<i32>,
    pub message: Option<String>,
    pub data: Option<Value>,
}

pub mod client {
    use lp_states::auth::{clear_token, get_token};
    use once_cell::sync::Lazy;
    use reqwest::{
        header::{self, AUTHORIZATION},
        StatusCode, Url,
    };
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
            let server_url = if let Some(app) = lp_states::config::get_config() {
                &app.app.remote_url
            } else {
                "127.0.0.1:5678"
            };

            let t = Url::parse(&server_url)
                .map_err(|e| anyhow::anyhow!("{:?}", e))?
                .join(&format!("/api/v1{}", resource))
                .map_err(|e| anyhow::anyhow!("{:?}", e))?;

            Ok(t)
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
            if let Ok(token) = reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token))
            {
                headers.insert(AUTHORIZATION, token);
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

pub async fn download_file(url: &str, dest: &str) -> Result<(), anyhow::Error> {
    let mut response = reqwest::Client::new().get(url).send().await?;
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("download {} resource fail.", url));
    }

    let total_size = response
        .headers()
        .get("Content-Length")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    let mut file = fs::File::create(Path::new(dest)).await?;
    let mut downloaded = 0u64;

    while let Some(chunk) = response.chunk().await? {
        downloaded += chunk.len() as u64;
        file.write_all(&chunk).await?;

        if total_size > 0 {
            let percent = (downloaded as f64 / total_size as f64) * 100.0;
            print!("\r下载进度: {:.2}%", percent);
            stdout().flush().await?;
        }
    }

    Ok(())
}

pub async fn iprust_info() -> Result<Value, anyhow::Error> {
    let url = "http://iprust.io/ip.json";
    let response = reqwest::Client::new().get(url).send().await?;
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("download {} resource fail.", url));
    }

    let res: Value = response.json().await?;

    Ok(res)
}

pub mod chrome_extension_scrapy {
    use regex::Regex;
    use reqwest::Client;
    use serde_json::{json, Value};

    pub fn parse_url_get_extension_uuid(url: &str) -> Result<&str, anyhow::Error> {
        let prefix = "chromewebstore.google.com/detail";

        if !url.contains(prefix) {
            return Err(anyhow::anyhow!("the url invalid."));
        }

        let re = Regex::new(r"/detail/[^/]+/([a-z0-9]{32})")?;

        if let Some(captures) = re.captures(url) {
            if let Some(extension_uuid) = captures.get(1) {
                return Ok(extension_uuid.as_str());
            }
        }
        return Err(anyhow::anyhow!("The url invalid."));
    }

    pub fn extension_download_url(chrome_version: &str, extension_uuid: &str) -> String {
        format!(
            "https://clients2.google.com/service/update2/crx?response=redirect&prodversion={}&acceptformat=crx2%2Ccrx3&x=id%3D{}%26uc",
            chrome_version, extension_uuid
        )
    }

    pub async fn extension_detail_by_url(url: &str) -> Result<Value, anyhow::Error> {
        let response = Client::new().get(url).send().await?;

        let response_text = response.text().await?;

        let mut json_value = json!({});

        let extension_uuid = parse_url_get_extension_uuid(url)?;
        json_value["extension_uuid"] = json!(extension_uuid);

        let title_re = Regex::new(r#"<h1 [^>]*>(.*?)</h1>"#).unwrap();
        if let Some(captures) = title_re.captures(&response_text) {
            if let Some(extension_title) = captures.get(1) {
                json_value["extension_title"] = json!(extension_title.as_str());
            }
        }

        let avatar_re =
            Regex::new(r#"<div[^>]*class="KgGEHd"[^>]*><img\ssrc=\"(.*?)\"\s\S.*</div>"#).unwrap();
        if let Some(captures) = avatar_re.captures(&response_text) {
            if let Some(extension_avatar) = captures.get(1) {
                json_value["extension_avatar"] = json!(extension_avatar.as_str());
            }
        }

        let description_re =
            Regex::new(r#"<div[^>]*class="RNnO5e"[^>]*>([\s\S]*?)</div>"#).unwrap();
        if let Some(div_cap) = description_re.captures(&response_text) {
            if let Some(div_content) = div_cap.get(1) {
                json_value["extension_description"] = json!(div_content.as_str());
            }
        }

        Ok(json_value)
    }

    #[test]
    fn test_parse_url_get_extension_uuid() {
        let extension_uuid = parse_url_get_extension_uuid(
            "https://chromewebstore.google.com/detail/sponsorblock-for-youtube/mnjggcdmjocbbbhaepdhchncahnbgone?hl=zh-CN&utm_source=ext_sidebar",
        );

        println!("{:?}", extension_uuid);
    }

    #[tokio::test]
    async fn test_extension_detail_by_url() {
        let urls = vec![
            // "https://chromewebstore.google.com/detail/evernote-web-clipper/pioclpoplcdbaefihamjohnefbikjilc",
            // "https://chromewebstore.google.com/detail/1930/mbcibgkijjmnhbbheafplbapiacgkebe?hl=zh-CN&utm_source=ext_sidebar",
            // "https://chromewebstore.google.com/detail/socialiq-influencer-marke/edpcocadldfbbpllhfkfcebnpigleamn?hl=zh-CN&utm_source=ext_sidebar",
            // "https://chromewebstore.google.com/detail/happy-dog-virtual-pet-for/cdoblkdcnbcdlcfklmbmkapbekgfbijp?hl=zh-CN&utm_source=ext_sidebar",
            // "https://chromewebstore.google.com/detail/pinball-space-adventure-g/iibghlclocgeljpbimnneepjceaiepkj?hl=zh-CN&utm_source=ext_sidebar"
            // "https://chromewebstore.google.com/detail/solid-devtools/kmcfjchnmmaeeagadbhoofajiopoceel?hl=zh-CN&utm_source=ext_sidebar"
            "https://chromewebstore.google.com/detail/axe-devtools-web-accessib/lhdoppojpmngadmnindnejefpokejbdd?hl=zh-CN&utm_source=ext_sidebar",
        ];

        for url in urls {
            let res = extension_detail_by_url(url).await.unwrap();

            println!("> {:?}", res);
        }
    }

    #[test]
    fn test_extension_download_url() {
        let extension_uuid = parse_url_get_extension_uuid("https://chromewebstore.google.com/detail/sponsorblock-for-youtube/mnjggcdmjocbbbhaepdhchncahnbgone?hl=zh-CN&utm_source=ext_sidebar").unwrap();

        let url = extension_download_url("131.0.6778.205", extension_uuid);
        println!("{:?}", url);
    }
}
