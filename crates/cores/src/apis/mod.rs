use std::sync::Arc;

use crate::{
    config::get_config,
    errors::ApplicationServerError,
    models::{self},
    utils::{command::Processer, fs::delete_data_file, response::AppResponse},
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

pub type Result<T> = core::result::Result<T, ApplicationServerError>;

lazy_static::lazy_static! {
     static ref  REMOTE_ENVIRONEMNT_TOTAL: Arc<Mutex<i64>> = Arc::new(Mutex::new(0)); // remote envs total by server
}

/// 分页查询
#[derive(Deserialize, Serialize, Debug)]
pub struct PageParam {
    pub page_num: Option<i32>,  // 页码，默认1，数量多需要翻页时用（可选）
    pub page_size: Option<i32>, // 每页大小，默认每页1 ，最大100（可选）
}

pub mod enviroment {
    use super::*;
    use crate::requests::backend;
    use models::enviroment::Environment;
    use serde_json::json;

    /// 从缓存中获取浏览器列表
    pub async fn get_browser_list_handle(
        payload: PageParam,
    ) -> Result<AppResponse<(i64, Vec<Environment>)>> {
        let (mut local_total, mut browsers) = Environment::query_envirionment(&payload).await?;
        let last_len = payload.page_num.unwrap_or_default() * payload.page_size.unwrap_or_default();
        let mut remote_total = REMOTE_ENVIRONEMNT_TOTAL.lock().await;
        if last_len as i64 > local_total || local_total == 0 {
            (local_total, browsers) = backend::environment::get_environment_list(&payload).await?;
            *remote_total = local_total; // this local_total is remote_total
            for env_item in &browsers {
                if let Ok(_) = Environment::insert(env_item.clone()).await {}
            }
        }
        Ok(AppResponse::success(None, Some((local_total, browsers))))
    }

    /// 从缓存中获取浏览器列表
    pub async fn get_browser_list_by_group_handle(
        group_id: i32,
        page_num: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<AppResponse<(i64, Vec<Environment>)>> {
        let (mut total, mut browsers) =
            Environment::query_envirionment_by_group_id(page_num, page_size, group_id).await?;
        let last_len = page_num.unwrap_or_default() * page_size.unwrap_or_default();
        if last_len as i64 >= total || total == 0 {
            (total, browsers) = backend::environment::get_environment_by_group_id(
                group_id,
                &PageParam {
                    page_num,
                    page_size,
                },
            )
            .await?;
            for env_item in &browsers {
                if let Ok(_) = Environment::insert(env_item.clone()).await {}
            }
        }

        Ok(AppResponse::success(None, Some((total, browsers))))
    }

    /// 获取单个环境
    pub async fn get_browser_by_id_handle(id: i32) -> Result<AppResponse<Option<Environment>>> {
        match Environment::query_envirionment_by_id(id).await {
            Ok(b) => Ok(AppResponse::success(None, Some(Some(b)))),
            Err(e) => match e {
                ApplicationServerError::DatabaseExecuteError(sqlx::Error::RowNotFound) => {
                    let browser_ = backend::environment::get_environment_by_id(id).await?;
                    Ok(AppResponse::success(None, Some(browser_)))
                }
                _ => Ok(AppResponse::success(None, Some(None))),
            },
        }
    }

    /// 更新浏览器环境
    pub async fn update_browser_handle(browser: Environment) -> Result<AppResponse<bool>> {
        let ok = backend::environment::update_environment(
            browser.id.unwrap_or_default(),
            browser.clone(),
        )
        .await?;
        if ok {
            Environment::update_environment(browser).await?;
        }

        Ok(AppResponse::success(None, Some(ok)))
    }

    /// 删除浏览器环境
    pub async fn delete_browser_handle(id: i32) -> Result<AppResponse<bool>> {
        let ok = backend::environment::delete_environment(id).await?;
        if ok {
            Environment::delete_envirionment(id).await?;
        }

        Ok(AppResponse::success(None, Some(ok)))
    }

    /// 添加浏览器环境
    pub async fn add_browser_handle(browser: Environment) -> Result<AppResponse<bool>> {
        let ok = backend::environment::create_environment(browser.clone()).await?;
        if ok {
            let mut remote_total = REMOTE_ENVIRONEMNT_TOTAL.lock().await;
            *remote_total += 1;
            Environment::insert(browser).await?;
        }

        Ok(AppResponse::success(None, Some(ok)))
    }

    /// 更新浏览器状态
    pub async fn update_browser_status_handle(id: i32, status: i8) -> Result<AppResponse<bool>> {
        let ok = Environment::update_envirionment_status(id, status).await?;
        Ok(AppResponse::success(None, Some(ok)))
    }

    pub async fn update_browser_proxy_handle(id: i32, proxy: &str) -> Result<AppResponse<bool>> {
        let ok =
            backend::environment::update_environment_by_json(id, json!({"proxy":proxy})).await?;
        if ok {
            Environment::update_envirionment_proxy(id, proxy).await?;
        }
        Ok(AppResponse::success(None, Some(ok)))
    }

    pub async fn update_browser_ua_handle(id: i32, ua: &str) -> Result<AppResponse<bool>> {
        let ok = backend::environment::update_environment_by_json(id, json!({"ua":ua})).await?;
        if ok {
            Environment::update_envirionment_proxy(id, ua).await?;
        }

        Ok(AppResponse::success(None, Some(ok)))
    }

    pub async fn update_browser_group_handle(id: i32, group_id: i32) -> Result<AppResponse<bool>> {
        let ok = backend::environment::update_environment_by_json(id, json!({"group_id":group_id}))
            .await?;
        if ok {
            Environment::update_envirionment_group(id, group_id).await?;
        }

        Ok(AppResponse::success(None, Some(ok)))
    }

    pub async fn update_browser_driver_location_handle(
        id: i32,
        location: &str,
    ) -> Result<AppResponse<bool>> {
        // TODO:
        let ok = Environment::update_envirionment_driver_location(id, location).await?;
        Ok(AppResponse::success(None, Some(ok)))
    }
}

// pub mod ua {
//     use models::ua::Ua;

//     use super::*;

//     /// 查询ua信息
//     pub fn list_ua_handle() -> Result<AppResponse<Vec<Ua>>> {
//         let ok = Ua::query_ua()?;
//         Ok(AppResponse::success(None, Some(ok)))
//     }

//     /// 更新ua信息
//     pub fn update_ua_handle(ua: Ua, id: i8) -> Result<AppResponse<bool>> {
//         let ok = Ua::update_ua(ua, id)?;
//         Ok(AppResponse::success(None, Some(ok)))
//     }

//     /// 删除ua信息
//     pub fn delete_ua_handle(id: i8) -> Result<AppResponse<bool>> {
//         let ok = Ua::delete_ua(id)?;
//         Ok(AppResponse::success(None, Some(ok)))
//     }

//     /// 添加ua信息
//     pub fn add_ua_handle(ua: Ua) -> Result<AppResponse<bool>> {
//         let ok = Ua::insert_ua(ua)?;
//         Ok(AppResponse::success(None, Some(ok)))
//     }
// }

pub mod group {

    use models::group::Group;

    use crate::requests::backend;

    use super::*;

    /// 查询group信息
    pub async fn list_group_handle(payload: PageParam) -> Result<AppResponse<(i64, Vec<Group>)>> {
        // TODO: 判断本地数据库是否存在，本地没有再尝试获取服务器
        let (mut total, mut groups) = Group::query_group(&payload).await?;

        let last_len = payload.page_num.unwrap_or_default() * payload.page_size.unwrap_or_default();
        if last_len as i64 >= total || total == 0 {
            (total, groups) = backend::group::get_group_list(&payload).await?;
            for env_item in &groups {
                if let Ok(_) = Group::insert_group(
                    &env_item.name,
                    &env_item.description.clone().unwrap_or_default(),
                )
                .await
                {}
            }
        }

        Ok(AppResponse::success(None, Some((total, groups))))
    }

    /// 更新group信息
    pub async fn update_group_handle(
        id: i32,
        group_name: &str,
        group_description: &str,
    ) -> Result<AppResponse<bool>> {
        let ok = backend::group::update_group(id, group_name, group_description).await?;
        if ok {
            Group::update_group(id, group_name, group_description).await?;
        }
        Ok(AppResponse::success(None, Some(ok)))
    }

    /// 删除group信息
    pub async fn delete_group_handle(id: i32) -> Result<AppResponse<bool>> {
        let ok = backend::group::delete_group(id).await?;
        if ok {
            Group::delete_group(id).await?;
        }
        Ok(AppResponse::success(None, Some(ok)))
    }

    /// 添加group信息
    pub async fn add_group_handle(
        group_name: &str,
        group_description: &str,
    ) -> Result<AppResponse<bool>> {
        let ok = backend::group::create_group(group_name, group_description).await?;
        if ok {
            Group::insert_group(group_name, group_description).await?;
        }

        Ok(AppResponse::success(None, Some(ok)))
    }
}

// pub mod gpu {
//     use models::gpu::Gpu;

//     use super::*;

//     /// 查询gpu信息
//     pub fn list_gpu_handle() -> Result<AppResponse<Vec<Gpu>>> {
//         let gpus = Gpu::query_gpu()?;
//         Ok(AppResponse::success(None, Some(gpus)))
//     }

//     /// 更新gpu信息
//     pub fn update_gpu_handle(gpu: Gpu, id: i8) -> Result<AppResponse<bool>> {
//         let ok = Gpu::update_gpu(gpu, id)?;
//         Ok(AppResponse::success(None, Some(ok)))
//     }

//     /// 删除gpu信息
//     pub fn delete_gpu_handle(id: i8) -> Result<AppResponse<bool>> {
//         let ok = Gpu::delete_gpu(id)?;
//         Ok(AppResponse::success(None, Some(ok)))
//     }

//     /// 添加gpu信息
//     pub fn add_gpu_handle(gpu: Gpu) -> Result<AppResponse<bool>> {
//         let ok = Gpu::insert_gpu(gpu)?;
//         Ok(AppResponse::success(None, Some(ok)))
//     }
// }

// pub mod cookie {
//     use models::cookie::PluginsCookie;

//     use super::*;

//     /// 查询cookie信息
//     pub fn list_gpu_handle(path: &str) -> Result<AppResponse<Vec<PluginsCookie>>> {
//         let cookies = PluginsCookie::query_cookie(path)?;
//         Ok(AppResponse::success(None, Some(cookies)))
//     }

//     /// 更新cookie信息
//     pub fn update_cookie_handle(cookie: PluginsCookie, path: &str) -> Result<AppResponse<bool>> {
//         let ok = PluginsCookie::update_cookie(cookie, path)?;
//         Ok(AppResponse::success(None, Some(ok)))
//     }

//     /// 删除cookie信息
//     pub fn delete_cookie_handle(path: &str, creation: &str) -> Result<AppResponse<bool>> {
//         let ok = PluginsCookie::delete_cookie(path, creation)?;
//         Ok(AppResponse::success(None, Some(ok)))
//     }

//     /// 添加cookie信息
//     pub fn add_cookie_handle(
//         cookie_path: &str,
//         cookie: PluginsCookie,
//     ) -> Result<AppResponse<bool>> {
//         let ok = PluginsCookie::insert_cookie(cookie_path, cookie)?;
//         Ok(AppResponse::success(None, Some(ok)))
//     }
// }

pub static ACTUATOR: Lazy<Arc<Mutex<Processer>>> =
    Lazy::new(|| Arc::new(Mutex::new(Processer::new())));

/// 操作浏览器的mod
pub mod browser {
    use std::collections::HashMap;

    use models::{enviroment::Environment, fingerprint::Fingerprint};

    use super::*;
    use crate::utils::{
        command::BrowserChildInfo,
        common::{get_chrome_install_path, get_debug_port},
    };

    #[derive(Debug, Deserialize, Serialize)]
    pub struct StartEnvironmentParams {
        environment: Environment,
        fingerprint: Fingerprint,
    }

    /// start browser
    /// get_proxy_from_registry
    pub async fn starts(
        payloads: Vec<StartEnvironmentParams>,
    ) -> Result<AppResponse<HashMap<i32, bool>>> {
        let mut data = HashMap::new();
        for payload in payloads {
            let port = get_debug_port().await?;

            let id = payload.environment.id.clone().unwrap_or_default();
            let browser_child_info = BrowserChildInfo::new(
                payload.environment,
                payload.fingerprint,
                port,
                &get_chrome_install_path().ok_or(ApplicationServerError::Error(
                    anyhow::anyhow!("chrome location get fail !"),
                ))?,
            );

            let ok = ACTUATOR
                .lock()
                .await
                .start_browser(browser_child_info)
                .await
                .map_err(|v| ApplicationServerError::Error(anyhow::anyhow!(v)))?;

            data.insert(id, ok.data.unwrap_or_default());
        }
        Ok(AppResponse::success(None, Some(data)))
    }

    // stop browser
    pub async fn stop(ids: Vec<i32>) -> Result<AppResponse<HashMap<i32, i32>>> {
        let mut data = HashMap::new();
        for id in ids {
            let statu = ACTUATOR.lock().await.stop_browser(id).await?.data;
            let code = statu.unwrap_or_default().code();
            data.insert(id, code.unwrap_or_default());
        }
        Ok(AppResponse::success(None, Some(data)))
    }

    // is active browser
    pub async fn is_active(id: i32) -> Result<AppResponse<bool>> {
        Ok(ACTUATOR.lock().await.status(id).await?)
    }

    // view active browser
    pub async fn view_active() -> Result<AppResponse<HashMap<i32, bool>>> {
        Ok(ACTUATOR.lock().await.all_status().await?)
    }
}

pub async fn delete_cache() -> Result<AppResponse<()>> {
    let data_cache_location = &get_config()?.setting.data_location;
    let user_data_cache_location = &get_config()?.setting.user_data_location;

    delete_data_file(&data_cache_location, "").await?;
    delete_data_file(&user_data_cache_location, "").await?;

    Ok(AppResponse::success(None, None))
}

pub mod browser_driver {
    use crate::Result;
    use std::{collections::HashMap, path::PathBuf};

    use crate::{config::get_config, requests};

    pub enum BrowserTye {
        Chrome,
        Firefox,
        Edge,
        Safi,
    }

    pub async fn get_all_version(t: BrowserTye) -> Result<Option<Vec<String>>> {
        match t {
            BrowserTye::Chrome => Ok(Some(
                requests::browser_resources::chrome::Action::new(
                    &get_config()?.browser.chrome.resource_url,
                )
                .await?
                .get_all_version(),
            )),
            _ => Ok(None),
        }
    }

    pub async fn get_current_platform(
        t: BrowserTye,
        version: &str,
    ) -> Result<Option<HashMap<String, Vec<HashMap<String, String>>>>> {
        match t {
            BrowserTye::Chrome => Ok(requests::browser_resources::chrome::Action::new(
                &get_config()?.browser.chrome.resource_url,
            )
            .await?
            .get_platform(version)),
            _ => Ok(None),
        }
    }

    pub async fn get_driver_download_where_location(t: &str, v: &str) -> Result<Option<String>> {
        let dirver_dir_location = get_config()?.get_brower_driver_location().await?;
        dirver_dir_location
            .join(PathBuf::from_iter(vec![t, v]))
            .to_str()
            .map_or(Ok(None), |v| Ok(Some(v.to_string())))
    }
}
