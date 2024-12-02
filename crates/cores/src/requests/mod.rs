pub mod backend;

use reqwest::Client;
use std::path::Path;
use tokio::{
    fs,
    io::{self, AsyncWriteExt},
};

use crate::{errors::ApplicationServerError, Result};

pub mod browser_resources {
    pub mod chrome {
        use crate::Result;
        use serde::Deserialize;
        use std::collections::HashMap;
        use tokio::sync::OnceCell;

        #[derive(Deserialize, Debug, Clone)]
        pub struct ChromeVersion {
            pub platform: String,
            pub url: String,
        }

        #[derive(Deserialize, Debug)]
        pub struct Milestone {
            pub milestone: String,
            pub version: String,
            pub revision: String,
            pub downloads: Downloads,
        }

        #[derive(Deserialize, Debug, Clone)]
        pub struct Downloads {
            pub chrome: Vec<ChromeVersion>,
        }

        #[derive(Deserialize, Debug)]
        pub struct ApiResponse {
            pub timestamp: String,
            pub milestones: HashMap<String, Milestone>,
        }

        pub struct Action {
            pub timestamp: String,
            pub platforms: HashMap<String, Vec<ChromeVersion>>,
            pub milestones: HashMap<String, String>,
        }

        pub static ACTION_CLINET: OnceCell<Action> = OnceCell::const_new();
        pub async fn init_action_client(url: &str) -> &'static Action {
            ACTION_CLINET
                .get_or_init(|| async {
                    let action = Action::new(url).await.unwrap();
                    action
                })
                .await
        }

        impl Action {
            pub async fn new(url: &str) -> Result<Action> {
                let response = reqwest::get(url).await?;
                if response.status().is_success() {
                    let response_json: ApiResponse = response.json().await?;
                    let mut platforms = HashMap::new();
                    let mut milestones = HashMap::new();
                    let timestamp = response_json.timestamp;

                    for key in response_json.milestones.keys() {
                        if let Some(chrome_version) = response_json.milestones.get(key) {
                            platforms
                                .insert(key.to_string(), chrome_version.downloads.chrome.clone());
                            milestones.insert(key.to_string(), chrome_version.version.to_string());
                        }
                    }
                    return Ok(Action {
                        timestamp,
                        platforms,
                        milestones,
                    });
                }
                Err(crate::errors::ApplicationServerError::BrowserDriverVersionInfoLoadFail)
            }

            pub fn get_all_version(&self) -> Vec<String> {
                let mut versions = Vec::new();
                for key in self.platforms.keys() {
                    versions.push(key.to_string());
                }
                versions
            }

            pub fn get_detail_version(&self, version: &str) -> Option<&String> {
                self.milestones.get(version)
            }

            pub fn get_platform(&self, version: &str) -> Option<Vec<HashMap<String, String>>> {
                let current_platform = self.platforms.get(version)?;

                Some(
                    current_platform
                        .iter()
                        .map(|v| {
                            let mut map = HashMap::new();
                            map.insert("platform".to_string(), v.platform.to_string());
                            map.insert("url".to_string(), v.url.to_string());
                            map
                        })
                        .collect(),
                )
            }
        }
    }
}

pub async fn download_file(url: &str, dest: &str) -> Result<()> {
    let mut response = Client::new().get(url).send().await?;
    if !response.status().is_success() {
        return Err(ApplicationServerError::Error(anyhow::anyhow!(
            "download {} resource fail.",
            url
        )));
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
            io::stdout().flush().await?;
        }
    }

    Ok(())
}
