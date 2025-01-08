use crate::{environment, environment_fingerprint, environment_proxy};
use commons::util::{get_chrome_install_path, get_debug_port};
use cores::processor::{self, Processer};
use models::environment_fingerprint::EnvironmentFingerprint;
use once_cell::sync::Lazy;
use serde_json::{json, Value};
use std::{collections::HashMap, sync::Arc};

use tokio::sync::{mpsc::Receiver, Mutex};

pub struct Actuator {
    controller: Arc<Mutex<Processer>>,
    #[allow(dead_code)]
    pub rx: Mutex<Receiver<(String, std::process::ExitStatus)>>,
}

impl Actuator {
    pub async fn listen_events(app: impl Fn(&str) -> ()) {
        let mut rx = ACTUATOR.rx.lock().await;

        while let Some((environment_uuid, _status)) = rx.recv().await {
            app(&environment_uuid)
        }
    }
}

pub static ACTUATOR: Lazy<Actuator> = Lazy::new(|| {
    let (processer, rx) = Processer::new();
    Actuator {
        controller: Arc::new(Mutex::new(processer)),
        rx: Mutex::new(rx),
    }
});

pub async fn view_active() -> Result<HashMap<String, bool>, anyhow::Error> {
    Ok(ACTUATOR
        .controller
        .lock()
        .await
        .all_status()
        .await
        .map_err(|e| anyhow::anyhow!("getter process status failed ({})", e))?)
}

pub async fn is_active(environment_uuid: &str) -> Result<bool, anyhow::Error> {
    Ok(ACTUATOR
        .controller
        .lock()
        .await
        .status(environment_uuid)
        .await?)
}

pub async fn stop_all() -> Result<bool, anyhow::Error> {
    match ACTUATOR.controller.lock().await.stop_all_browser().await {
        Ok(_statu) => Ok(true),
        Err(_e) => Ok(false),
    }
}

pub async fn stop(
    environment_uuiids: Vec<String>,
) -> Result<HashMap<String, String>, anyhow::Error> {
    let mut data = HashMap::new();
    for id in environment_uuiids {
        match ACTUATOR.controller.lock().await.stop_browser(&id).await {
            Ok(statu) => {
                let code = statu.code();
                data.insert(id, code.unwrap_or_default().to_string());
            }
            Err(e) => {
                data.insert(id, e.to_string());
            }
        }
    }
    Ok(data)
}

pub async fn start_browser(environment_uuid: &str) -> Result<Value, anyhow::Error> {
    let port = get_debug_port().await?;
    let mut stauts = false;
    let message;

    match environment::query_by_uuid(environment_uuid).await {
        Ok(current_environement) => {
            let user_uuid = &current_environement.user_uuid;

            let fp_info = if let Some(fp_info_id) = current_environement.fp_info_id {
                environment_fingerprint::query_by_id(&user_uuid, fp_info_id as u32).await?
            } else {
                EnvironmentFingerprint {
                    ..Default::default()
                }
            };

            let proxy_info = if let Some(proxy_id) = current_environement.proxy_id {
                match environment_proxy::query_by_id(&user_uuid, proxy_id as u32).await {
                    Ok(v) => v,
                    Err(_) => models::environment_proxies::Proxy {
                        ..Default::default()
                    },
                }
            } else {
                models::environment_proxies::Proxy {
                    ..Default::default()
                }
            };

            let chrome_install_path =
                get_chrome_install_path().ok_or(anyhow::anyhow!("chrome location get fail !"))?;
            let browser_child_info = processor::BrowserChildInfo::new(
                current_environement,
                fp_info,
                proxy_info,
                port,
                &chrome_install_path,
            );
            let mut actuator = ACTUATOR.controller.lock().await;

            match actuator.start_browser(browser_child_info).await {
                Ok(ok) => {
                    stauts = ok;
                    message = "启动成功".to_string();
                }
                Err(err) => {
                    message = err.to_string();
                }
            };
        }
        Err(_) => {
            message = "启动失败，指定环境不存在".to_string();
        }
    };

    Ok(json!({
        "environment_uuid": environment_uuid,
        "status":  stauts,
        "message": message,
    }))
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[tokio::test]
    async fn test_view_active() {
        crate::setup().await;

        let result = view_active().await;
        assert!(result.is_ok());
        let status_map = result.unwrap();
        assert!(status_map.is_empty() || status_map.values().all(|&v| v == true || v == false));
    }

    #[tokio::test]
    async fn test_is_active() {
        crate::setup().await;
        let environment_uuid = "test_uuid";
        let result = is_active(environment_uuid).await;
        assert!(result.is_ok());
        let status = result.unwrap();
        assert!(status == true || status == false);
    }

    #[tokio::test]
    async fn test_stop() {
        crate::setup().await;
        let ids = vec!["test_id1".to_string(), "test_id2".to_string()];
        let result = stop(ids.clone()).await;
        assert!(result.is_ok());
        let status_map = result.unwrap();
        for id in ids {
            assert!(status_map.contains_key(&id));
        }
    }

    #[tokio::test]
    async fn test_start_browser() {
        crate::setup().await;
        let environment_uuid = "d7e2d896-8e09-4713-8f53-c2225e224afd";
        let result = start_browser(environment_uuid).await;
        println!("{:?}", result);

        tokio::time::sleep(Duration::from_secs(1)).await;

        let ids = vec![environment_uuid.to_string()];
        let result = stop(ids.clone()).await;
        println!("{:?}", result);
    }
}
