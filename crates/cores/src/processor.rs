use std::{
    collections::HashMap,
    process::{ExitStatus, Stdio},
    sync::Arc,
};

use tokio::{
    process::{Child, Command},
    sync::Mutex,
};

use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub struct BrowserChildInfo {
    environemnt_info: models::environment::Environment,
    fingerprint_info: models::environment_fingerprint::EnvironmentFingerprint,
    proxy_info: models::environment_proxies::Proxy,
    pub port: u16,
    pub browser_exe_path: String,
}
impl BrowserChildInfo {
    pub fn new(
        environemnt_info: models::environment::Environment,
        fingerprint_info: models::environment_fingerprint::EnvironmentFingerprint,
        proxy_info: models::environment_proxies::Proxy,
        port: u16,
        browser_exe_path: &str,
    ) -> Self {
        BrowserChildInfo {
            environemnt_info,
            fingerprint_info,
            proxy_info,
            port,
            browser_exe_path: browser_exe_path.to_string(),
        }
    }

    pub async fn format(&self) -> Result<Vec<String>, anyhow::Error> {
        let breeze_fp = format!(
            "--breeze-fp={}",
            commons::encryption::base64_encode(&serde_json::to_string(&self.fingerprint_info)?)
        );
        let new_window = "--new-window".to_string();
        let window_size = format!(
            "--window-size={},{}",
            self.fingerprint_info.width.unwrap_or_default(),
            self.fingerprint_info.height.unwrap_or_default()
        );
        let window_position = format!(
            "--window-position={},{}",
            self.fingerprint_info.longitude.clone().unwrap_or_default(),
            self.fingerprint_info.latitude.clone().unwrap_or_default(),
        );
        let user_agent = format!("--user-agent={}", self.fingerprint_info.ua);
        let accept_lang = format!("--accept-lang={}", self.fingerprint_info.languages);
        let no_first_run = "--no-first-run".to_string();

        let app_config = states::config::get_config().unwrap();
        let current_dir = std::env::current_dir().unwrap();
        let app_data = states::config::APP_DATA.clone();
        let app_data = app_data
            .to_str()
            .unwrap_or_else(|| current_dir.to_str().unwrap());
        let user_data_dir = format!(
            r#"--user-data-dir={}/{}/{}"#,
            app_data,
            app_config.app.location.user_data_location,
            commons::time::get_system_time_mills(),
        );

        let no_default_browser_check = "--no-default-browser-check".to_string();
        let browser_unique = format!(
            "--app-browser-unique={}.{}",
            app_config.app.id,
            format!(
                "{}.{}",
                self.environemnt_info.uuid.clone().unwrap_or_default(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_micros()
            )
        );
        let debugger_address = format!("--remote-debugging-port={}", self.port);

        let mut args: Vec<String> = vec![
            no_default_browser_check,
            no_first_run,
            window_size,
            window_position,
            new_window,
            accept_lang,
            user_agent,
            user_data_dir,
            breeze_fp,
            browser_unique,
            debugger_address,
        ];

        if self.environemnt_info.proxy_enable == 1 {
            let (kind, value) = (
                &self.proxy_info.kind,
                format!("{}:{}", &self.proxy_info.host, &self.proxy_info.port),
            );

            args.push(if !value.is_empty() {
                format!("--proxy-server={}://{}", kind, value,)
            } else {
                format!(
                    "--proxy-server=socks5://{}",
                    commons::util::get_proxy_from_registry().unwrap_or_default()
                )
            });
        }

        for url in self
            .environemnt_info
            .default_urls
            .clone()
            .unwrap_or_default()
            .split(",")
        {
            args.push(url.to_string());
        }

        Ok(args)
    }
}

#[allow(dead_code)]
pub struct Processer {
    childs: Arc<Mutex<HashMap<u32, (Child, BrowserChildInfo)>>>,
    index: HashMap<String, u32>,
}

impl Processer {
    pub fn new() -> Self {
        Processer {
            childs: Arc::new(Mutex::new(HashMap::new())),
            index: HashMap::new(),
        }
    }

    pub async fn start_browser(
        &mut self,
        payload: BrowserChildInfo,
    ) -> core::result::Result<bool, anyhow::Error> {
        let child = Command::new(&payload.browser_exe_path)
            .args(&payload.format().await?)
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .spawn()?;

        let browser_id = &payload.environemnt_info.uuid.clone().unwrap_or_default();

        let child_pid = child
            .id()
            .ok_or(anyhow::anyhow!("child starting failed."))?;

        self.index.insert(browser_id.to_string(), child_pid);
        {
            self.childs
                .clone()
                .lock()
                .await
                .insert(child_pid, (child, payload));
        }

        Ok(true)
    }

    pub async fn stop_browser(&mut self, browser_uuid: &str) -> Result<ExitStatus, anyhow::Error> {
        let child_id = self
            .index
            .get(browser_uuid)
            .ok_or(anyhow::anyhow!("child stoping failed."))?;

        let exit_status = {
            let mut childs_lock = self.childs.lock().await;

            let (child, _) = childs_lock
                .get_mut(child_id)
                .ok_or(anyhow::anyhow!("child stoping failed."))?;

            let _ = child.kill().await?;
            child.wait().await?
        };

        Ok(exit_status)
    }

    pub async fn status(&self, browser_uuid: &str) -> Result<bool, anyhow::Error> {
        let child_id = self
            .index
            .get(browser_uuid)
            .ok_or(anyhow::anyhow!("child status failed."))?;

        let status = {
            let mut childs_lock = self.childs.lock().await;

            let (child, _) = childs_lock
                .get_mut(child_id)
                .ok_or(anyhow::anyhow!("child status failed."))?;

            match child.try_wait() {
                Ok(Some(_)) => false,
                Ok(None) => true,
                Err(_) => false,
            }
        };

        Ok(status)
    }

    pub async fn all_status(&self) -> Result<HashMap<String, bool>, anyhow::Error> {
        let mut status = HashMap::new();
        println!("{:?}", &self.index);

        for (browser_uuid, _) in &self.index {
            let data = self.status(&*&browser_uuid).await?;

            status.insert(browser_uuid.clone(), data);
        }

        Ok(status)
    }
}
