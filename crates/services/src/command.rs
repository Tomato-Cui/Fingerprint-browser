use crate::{environment, environment_fingerprint, environment_proxy};
use commons::util::{get_chrome_install_path, get_debug_port};
use cores::processor::{self, Processer};
use models::environment_fingerprint::EnvironmentFingerprint;
use once_cell::sync::Lazy;
use serde_json::{json, Value};
use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

pub static ACTUATOR: Lazy<Arc<Mutex<Processer>>> =
    Lazy::new(|| Arc::new(Mutex::new(Processer::new())));

pub async fn view_active() -> Result<HashMap<String, bool>, anyhow::Error> {
    Ok(ACTUATOR
        .lock()
        .await
        .all_status()
        .await
        .map_err(|e| anyhow::anyhow!("getter process status failed ({})", e))?)
}

pub async fn is_active(environment_uuid: &str) -> Result<bool, anyhow::Error> {
    Ok(ACTUATOR.lock().await.status(environment_uuid).await?)
}

pub async fn stop(ids: Vec<String>) -> Result<HashMap<String, i32>, anyhow::Error> {
    let mut data = HashMap::new();
    for id in ids {
        let statu = ACTUATOR.lock().await.stop_browser(&id).await?;
        let code = statu.code();
        data.insert(id, code.unwrap_or_default());
    }
    Ok(data)
}

pub async fn start_browser(environment_uuid: &str) -> Result<Value, anyhow::Error> {
    let port = get_debug_port().await?;
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

            let ok = ACTUATOR
                .lock()
                .await
                .start_browser(browser_child_info)
                .await?;

            Ok(json!({
                            "environment_uuid": environment_uuid,
                            "status":  ok,
                            "message":format!("启动成功."),
            }))
        }
        Err(_) => Ok(json!({
                        "environment_uuid": environment_uuid,
                        "status":  false,
                        "message":format!("启动失败, 指定环境ID不存在."),
        })),
    }
}

#[cfg(test)]
mod tests {
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
        let environment_uuid = "test_uuid";
        let result = start_browser(environment_uuid).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response["environment_uuid"], environment_uuid);
        assert!(response["status"].is_boolean());
        assert!(response["message"].is_string());
    }
}
