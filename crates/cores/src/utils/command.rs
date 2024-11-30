use std::{
    collections::HashMap,
    process::{ExitStatus, Stdio},
    sync::Arc,
};

use tokio::{
    process::{Child, Command},
    sync::Mutex,
};

pub use crate::errors::ApplicationServerError;
use crate::{
    apis::{enviroment::update_browser_status_handle, Result},
    config,
    models::enviroment::Browser,
    utils::{
        common::{app_localer, to_string},
        encryption,
    },
};

use crate::utils::common::get_proxy_from_registry;

use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use super::response::AppResponse;

pub struct BrowserChildInfo {
    browser_info: Browser,
    pub port: u16,
    pub browser_exe_path: String,
}
impl BrowserChildInfo {
    pub fn new(browser_info: Browser, port: u16, browser_exe_path: &str) -> Self {
        BrowserChildInfo {
            browser_info,
            port,
            browser_exe_path: browser_exe_path.to_string(),
        }
    }

    pub fn format(&self) -> Result<Vec<String>> {
        let breeze_fp = format!(
            "--breeze-fp={}",
            encryption::base64_encode(&to_string(&self.browser_info.fp_info)?)
        );
        let new_window = "--new-window".to_string();
        let window_size = format!(
            "--window-size={},{}",
            self.browser_info.fp_info.h, self.browser_info.fp_info.w
        );
        let window_position = format!(
            "--window-position={},{}",
            self.browser_info.fp_info.la, self.browser_info.fp_info.lo
        );
        let user_agent = format!("--user-agent={}", self.browser_info.ua);
        let accept_lang = format!("--accept-lang={}", self.browser_info.fp_info.lang);
        let no_first_run = "--no-first-run".to_string();

        let user_data_dir = format!(
            "--user-data-dir={}",
            config::get_config()?
                .get_user_data_location()?
                .join(config::get_config()?.get_user_data_location()?)
                .join(&self.browser_info.user_data_file)
                .to_str()
                .unwrap()
        );
        let no_default_browser_check = "--no-default-browser-check".to_string();
        let browser_unique = format!(
            "--app-browser-unique={}.{}",
            config::get_config()?.app.id,
            format!(
                "{}.{}",
                self.browser_info.id.unwrap_or_default(),
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

        let proxy_str = self
            .browser_info
            .proxy
            .clone()
            .unwrap_or_else(|| "".to_string());

        if self.browser_info.proxy_enable {
            args.push(if proxy_str.is_empty() {
                format!(
                    "--proxy-server=socks5://{}",
                    get_proxy_from_registry().unwrap_or_default()
                )
            } else {
                let proxy = get_proxy_from_registry().unwrap_or_default();

                if !proxy.is_empty() {
                    format!(
                        "--proxy-server=socks5://{}",
                        get_proxy_from_registry().unwrap_or_default()
                    )
                } else {
                    "".to_string()
                }
            })
        }
        if let Some(urls) = &self.browser_info.open_urls {
            urls.iter().for_each(|v| {
                args.push(v.to_string());
            });
        }

        Ok(args)
    }
}

/// 执行器
#[allow(dead_code)]
pub struct Processer {
    childs: Arc<Mutex<HashMap<u32, (Child, BrowserChildInfo)>>>,
    index: HashMap<i8, u32>, // i8: browser_id, u32: child pid
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
    ) -> core::result::Result<AppResponse<bool>, ApplicationServerError> {
        let browser_path = app_localer::app_location()
            // TODO: WANR: 这里我随便写一个路径都能通过测试.join("test")
            // .join("BreezeBrowser")
            .join(&payload.browser_exe_path);

        let child = Command::new(browser_path)
            .args(&payload.format()?)
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .spawn()?;

        let browser_id = payload.browser_info.id.unwrap_or_default();
        update_browser_status_handle(browser_id, true)?;
        let child_pid = child
            .id()
            .ok_or(ApplicationServerError::ChildRunningError)?;

        self.index.insert(browser_id, child_pid);
        {
            self.childs
                .clone()
                .lock()
                .await
                .insert(child_pid, (child, payload));
        }

        Ok(AppResponse::success(None, Some(true)))
    }

    pub async fn stop_browser(&mut self, browser_id: i8) -> Result<AppResponse<ExitStatus>> {
        let child_id = self
            .index
            .get(&browser_id)
            .ok_or(ApplicationServerError::ChildCloseError)?;

        let exit_status = {
            let mut childs_lock = self.childs.lock().await;

            let (child, _) = childs_lock
                .get_mut(child_id)
                .ok_or(ApplicationServerError::ChildCloseError)?;

            let _ = child.kill().await?;
            child.wait().await?
        };

        update_browser_status_handle(browser_id, false)?;
        Ok(AppResponse::success(
            Some("close browser finally !".to_string()),
            Some(exit_status),
        ))
    }

    pub async fn status(&self, browser_id: i8) -> Result<AppResponse<bool>> {
        let child_id = self
            .index
            .get(&browser_id)
            .ok_or(ApplicationServerError::ChildCloseError)?;

        let status = {
            let mut childs_lock = self.childs.lock().await;

            let (child, _) = childs_lock
                .get_mut(child_id)
                .ok_or(ApplicationServerError::ChildCloseError)?;

            match child.try_wait() {
                Ok(Some(_)) => false,
                Ok(None) => true,
                Err(_) => false,
            }
        };

        update_browser_status_handle(browser_id, status)?;
        Ok(AppResponse::success(
            Some("select browser status !".to_string()),
            Some(status),
        ))
    }

    pub async fn all_status(&self) -> Result<AppResponse<HashMap<i8, bool>>> {
        let mut status = HashMap::new();

        for (browser_id, _) in &self.index {
            let data = self.status(*browser_id).await?.data;
            let data = data.unwrap_or_default();

            update_browser_status_handle(*browser_id, data)?;
            status.insert(*browser_id, data);
        }

        Ok(AppResponse::success(
            Some("select browsers status finally !".to_string()),
            Some(status),
        ))
    }
}
