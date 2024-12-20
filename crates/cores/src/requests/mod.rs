use reqwest::Client;
use std::path::Path;
use tokio::{
    fs,
    io::{self, AsyncWriteExt},
};

pub mod browser_resources {
    pub mod chrome {
        use serde::Deserialize;
        use std::collections::HashMap;
        use tokio::sync::OnceCell;

        #[derive(Deserialize, Debug, Clone)]
        pub struct ChromeVersion {
            pub platform: String,
            pub url: String,
        }

        #[derive(Deserialize, Debug)]
        pub struct Version {
            // pub milestone: String,
            pub version: String,
            pub revision: String,
            pub downloads: Downloads,
        }

        #[derive(Deserialize, Debug, Clone)]
        pub struct Downloads {
            pub chrome: Vec<ChromeVersion>,
            pub chromedriver: Option<Vec<ChromeVersion>>,
            #[serde(rename = "chrome-headless-shell")]
            pub chrome_headless_shell: Option<Vec<ChromeVersion>>,
        }

        #[derive(Deserialize, Debug)]
        pub struct ApiResponse {
            pub timestamp: String,
            pub versions: Vec<Version>,
        }

        pub struct Action {
            pub timestamp: String,
            pub platforms: HashMap<String, Vec<ChromeVersion>>,
            pub platform_drivers: HashMap<String, Option<Vec<ChromeVersion>>>,
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
            pub async fn new(url: &str) -> Result<Action, anyhow::Error> {
                let response = reqwest::get(url).await?;
                if response.status().is_success() {
                    let response_json: ApiResponse = response.json().await?;
                    let mut platforms = HashMap::new();
                    let mut platform_drivers = HashMap::new();
                    let mut milestones = HashMap::new();
                    let timestamp = response_json.timestamp;

                    for version in response_json.versions {
                        let key = &version.version;
                        platforms.insert(key.to_string(), version.downloads.chrome.clone());
                        platform_drivers
                            .insert(key.to_string(), version.downloads.chromedriver.clone());
                        milestones.insert(key.to_string(), version.version.to_string());
                    }
                    return Ok(Action {
                        timestamp,
                        platforms,
                        platform_drivers,
                        milestones,
                    });
                }
                Err(anyhow::anyhow!("get browser resources fail."))
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

            pub fn get_platform(
                &self,
                version: &str,
            ) -> Option<HashMap<String, Vec<HashMap<String, String>>>> {
                let current_platform = self.platforms.get(version)?;
                let current_platform_driver = self.platform_drivers.get(version)?;

                let current_platform: Vec<HashMap<String, String>> = current_platform
                    .iter()
                    .map(|v| {
                        let mut map = HashMap::new();

                        map.insert("platform".to_string(), v.platform.to_string());
                        map.insert("url".to_string(), v.url.to_string());
                        map
                    })
                    .collect();

                let current_platform_driver = if let Some(drivers) = current_platform_driver {
                    drivers
                        .iter()
                        .map(|v| {
                            let mut map = HashMap::new();
                            map.insert("platform".to_string(), v.platform.to_string());
                            map.insert("url".to_string(), v.url.to_string());
                            map
                        })
                        .collect()
                } else {
                    Vec::new()
                };

                let mut map = HashMap::new();
                map.insert("chrome".to_string(), current_platform);
                map.insert("chrome-driver".to_string(), current_platform_driver);

                Some(map)
            }
        }
    }
}

pub async fn download_file(url: &str, dest: &str) -> Result<(), anyhow::Error> {
    let mut response = Client::new().get(url).send().await?;
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
            io::stdout().flush().await?;
        }
    }

    Ok(())
}
