use std::{
    collections::HashMap,
    process::{ExitStatus, Stdio},
    sync::Arc,
};
use tokio::{
    process::{Child, Command},
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex,
    },
};

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
            self.fingerprint_info.width.unwrap_or_else(|| 888),
            self.fingerprint_info.height.unwrap_or_else(|| 888)
        );

        let system_time_millis = commons::time::get_system_time_mills();
        let offset = (system_time_millis % 100) as i32;

        let window_position = format!(
            "--window-position={},{}",
            self.fingerprint_info
                .longitude
                .unwrap_or_else(|| 100 + offset),
            self.fingerprint_info
                .latitude
                .unwrap_or_else(|| 100 + offset),
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
            "--user-data-dir={}/{}/{}",
            app_data,
            app_config.app.location.user_data_location,
            self.environemnt_info.uuid.clone().unwrap_or_default(),
        );

        let no_default_browser_check = "--no-default-browser-check".to_string();
        let browser_unique = format!(
            "--app-browser-unique={}.{}",
            app_config.app.id,
            self.environemnt_info.uuid.clone().unwrap_or_default(),
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
            .unwrap_or_else(|| "https://www.google.com".to_string())
            .split(",")
        {
            args.push(url.to_string());
        }

        Ok(args)
    }
}

#[allow(dead_code)]
pub struct Processer {
    childs: Arc<Mutex<HashMap<u32, (Arc<Mutex<Child>>, BrowserChildInfo)>>>,
    index: Arc<Mutex<HashMap<String, u32>>>,
    tx: Sender<(String, ExitStatus)>,
}

impl Processer {
    pub fn new() -> (Self, Receiver<(String, ExitStatus)>) {
        let childs = Arc::new(Mutex::new(HashMap::new()));
        let index = Arc::new(Mutex::new(HashMap::new()));
        let (tx, rx) = mpsc::channel::<(String, ExitStatus)>(32);

        (Processer { childs, index, tx }, rx)
    }

    pub async fn start_browser(
        &mut self,
        payload: BrowserChildInfo,
    ) -> core::result::Result<bool, anyhow::Error> {
        let environment_uuid = payload.environemnt_info.clone().uuid.unwrap_or_default();
        if let Ok(ok) = self.status(&environment_uuid).await {
            if ok {
                return Ok(ok);
            }
        }

        let browser_exe_path = payload.browser_exe_path.clone();
        let childs = self.childs.clone();
        let indexs = self.index.clone();
        let tx = self.tx.clone();

        tokio::spawn(async move {
            let child = Command::new(&browser_exe_path)
                .args(&payload.format().await?)
                .stderr(Stdio::null())
                .stdout(Stdio::null())
                .spawn()?;

            let child_pid = child
                .id()
                .ok_or(anyhow::anyhow!("child starting failed."))?;

            let child_arc = Arc::new(Mutex::new(child));
            {
                indexs
                    .lock()
                    .await
                    .insert(environment_uuid.clone(), child_pid);
                childs
                    .lock()
                    .await
                    .insert(child_pid, (Arc::clone(&child_arc), payload));
            }

            loop {
                tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

                if let Ok(mut child) = child_arc.try_lock() {
                    if let Ok(Some(status)) = child.try_wait() {
                        let _ = tx.send((environment_uuid.clone(), status)).await;
                        break;
                    }
                }
            }

            Ok::<(), anyhow::Error>(())
        });
        Ok(true)
    }

    pub async fn stop_all_browser(&mut self) -> Result<Vec<ExitStatus>, anyhow::Error> {
        let mut childs_lock = self.childs.lock().await;
        let indexs = self.index.clone();
        let mut index_lock = indexs.lock().await;
        let mut exit_status = vec![];

        for (_, child_pid) in &*index_lock {
            if let Some((child, _)) = childs_lock.get_mut(&child_pid) {
                let mut child = child.lock().await;
                // .ok_or(anyhow::anyhow!("child stoping failed."))?;
                let a = child.kill().await;
                println!("{:?}", a);
                exit_status.push(child.wait().await?);
            } else {
                println!("获取指纹失败")
            }
        }

        let keys: Vec<String> = index_lock.keys().cloned().collect();
        for browser_uuid in keys {
            index_lock.remove(&browser_uuid);
        }

        Ok(exit_status)
    }

    pub async fn stop_browser(&mut self, browser_uuid: &str) -> Result<ExitStatus, anyhow::Error> {
        let index = self.index.clone();
        let mut index_lock = index.lock().await;
        let child_pid: &u32 = index_lock
            .get(browser_uuid)
            .ok_or(anyhow::anyhow!("child stoping failed."))?;

        let exit_status = {
            let mut childs_lock = self.childs.lock().await;

            if let Some((child, _)) = childs_lock.get_mut(&child_pid) {
                let mut child = child.lock().await;
                let _ = child.kill().await.expect("abc");
                index_lock.remove(browser_uuid);
                child.wait().await?
            } else {
                return Err(anyhow::anyhow!("child stopping failed: PID not found"));
            }
        };

        Ok(exit_status)
    }

    pub async fn status(&self, browser_uuid: &str) -> Result<bool, anyhow::Error> {
        let index = self.index.clone();
        let index_lock = index.lock().await;
        let child_pid = index_lock
            .get(browser_uuid)
            .ok_or(anyhow::anyhow!("child status failed."))?;

        let status = {
            let mut childs_lock = self.childs.lock().await;

            let (child, _) = childs_lock
                .get_mut(child_pid)
                .ok_or(anyhow::anyhow!("child status failed."))?;
            let mut child = child.lock().await;

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
        let indexs = self.index.clone();

        for (browser_uuid, _) in &*indexs.lock().await {
            let data = self.status(&*&browser_uuid).await?;

            status.insert(browser_uuid.clone(), data);
        }

        Ok(status)
    }
}
