use crate::{environment, error::ServiceError, fingerprint, proxy};
use commons::util::{get_chrome_install_path, get_debug_port};
use cores::processor::{self, Processer};
use once_cell::sync::Lazy;
use serde_json::{json, Value};
use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

pub async fn clean_cache() -> Result<bool, ServiceError> {
    // TODO: 获取到路径
    Ok(true)
}

pub static ACTUATOR: Lazy<Arc<Mutex<Processer>>> =
    Lazy::new(|| Arc::new(Mutex::new(Processer::new())));

pub async fn view_active() -> Result<HashMap<i32, bool>, anyhow::Error> {
    Ok(ACTUATOR
        .lock()
        .await
        .all_status()
        .await
        .map_err(|e| anyhow::anyhow!("getter process status failed ({})", e))?)
}

pub async fn is_active(id: i32) -> Result<bool, anyhow::Error> {
    Ok(ACTUATOR.lock().await.status(id).await?)
}

pub async fn stop(ids: Vec<i32>) -> Result<HashMap<i32, i32>, anyhow::Error> {
    let mut data = HashMap::new();
    for id in ids {
        let statu = ACTUATOR.lock().await.stop_browser(id).await?;
        let code = statu.code();
        data.insert(id, code.unwrap_or_default());
    }
    Ok(data)
}

pub async fn start_browser(
    user_id: Option<u32>,
    group_id: Option<u32>,
    environment_id: u32,
) -> Result<Value, anyhow::Error> {
    let port = get_debug_port().await?;
    match environment::query_by_id(user_id, group_id, environment_id).await {
        Ok(current_environement) => {
            let fp_info = if let Some(user_id) = current_environement.owner_id {
                fingerprint::query_by_id(
                    user_id as u32,
                    current_environement.fp_info_id.unwrap_or_default() as u32,
                )
                .await?
            } else {
                fingerprint::default().await?
            };

            let proxy_info = if let Some(user_id) = current_environement.owner_id {
                proxy::query_by_env_id(user_id as u32, environment_id).await?
            } else {
                models::proxies::Proxy {
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
                            "environment_id": environment_id,
                            "status":  ok,
                            "message":format!("启动成功."),
            }))
        }
        Err(_) => Ok(json!({
                        "environment_id": environment_id,
                        "status":  false,
                        "message":format!("启动失败, 指定环境ID不存在."),
        })),
    }
}
