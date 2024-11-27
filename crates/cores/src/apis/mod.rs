use crate::{
    config::AConfig,
    errors::ApplicationServerError,
    models::{self},
    utils::{fs::delete_data_file, response::AppResponse},
};

pub type Result<T> = core::result::Result<T, ApplicationServerError>;

use serde::{Deserialize, Serialize};
/// 分页查询
#[derive(Deserialize, Serialize, Debug)]
pub struct PageParam {
    pub page_num: Option<u8>,  // 页码，默认1，数量多需要翻页时用（可选）
    pub page_size: Option<u8>, // 每页大小，默认每页1 ，最大100（可选）
}

pub mod enviroment {
    use super::*;
    use crate::utils::common::to_string;
    use models::{enviroment::Browser, ua::Ua};

    /// 获取浏览器列表
    pub fn get_browser_list_handle(payload: PageParam) -> Result<AppResponse<Vec<Browser>>> {
        let browsers = Browser::query_browser(payload)?;
        // TODO: 判断本地数据库是否存在，本地没有再尝试获取服务器
        Ok(AppResponse::success(None, Some(browsers)))
    }

    /// 更新浏览器环境
    pub fn update_browser_handle(browser: Browser) -> Result<AppResponse<bool>> {
        // TODO: 先更新到服务器，等服务器成功后再同步到本地
        // let browser_info: Browser = serde_json::from_str(browser)?;
        let ok = Browser::update_browser(browser)?;
        Ok(AppResponse::success(None, Some(ok)))
    }

    /// 删除浏览器环境
    pub fn delete_browser_handle(ids: Vec<u8>) -> Result<AppResponse<bool>> {
        // TODO: 先删除服务器中的，等服务器成功后再同步到本地
        let ok = Browser::delete_browser(ids)?;
        Ok(AppResponse::success(None, Some(ok)))
    }

    /// 添加浏览器环境
    pub fn add_browser_handle(browser: Browser) -> Result<AppResponse<bool>> {
        // TODO: 先添加到服务器，等服务器成功后再同步到本地
        // let browser_info: Browser = serde_json::from_str(browser)?;
        let ok = Browser::insert_browser(browser)?;
        Ok(AppResponse::success(None, Some(ok)))
    }

    /// 更新浏览器状态
    pub fn update_browser_status_handle(id: i8, status: bool) -> Result<AppResponse<bool>> {
        let ok = models::enviroment::Browser::update_browser_status(id, status)?;
        Ok(AppResponse::success(None, Some(ok)))
    }

    /// 更新浏览器状态
    pub fn update_browser_proxy_handle(id: i8, proxy: Option<&str>) -> Result<AppResponse<bool>> {
        let ok = models::enviroment::Browser::update_browser_proxy(id, proxy)?;
        Ok(AppResponse::success(None, Some(ok)))
    }

    /// 更新浏览器状态
    pub fn update_browser_ua_handle(id: i8, ua: Option<Ua>) -> Result<AppResponse<bool>> {
        let ua_str = to_string(ua)?;
        if ua_str.is_empty() {
            return Ok(AppResponse::success(
                Some("ua is empty".to_string()),
                Some(false),
            ));
        }

        let ok = models::enviroment::Browser::update_browser_ua(id, Some(&ua_str))?;
        Ok(AppResponse::success(None, Some(ok)))
    }

    /// 更新浏览器分组
    pub fn update_browser_group_handle(id: u8, group_id: &str) -> Result<AppResponse<bool>> {
        let ok = models::enviroment::Browser::update_browser_group(id, group_id)?;
        Ok(AppResponse::success(None, Some(ok)))
    }
}

pub mod ua {
    use models::ua::Ua;

    use super::*;

    /// 查询ua信息
    pub fn list_ua_handle() -> Result<AppResponse<Vec<Ua>>> {
        // TODO: 判断本地数据库是否存在，本地没有再尝试获取服务器
        let ok = Ua::query_ua()?;
        Ok(AppResponse::success(None, Some(ok)))
    }

    /// 更新ua信息
    pub fn update_ua_handle(ua: Ua, id: i8) -> Result<AppResponse<bool>> {
        // TODO: 先更新到服务器，等服务器成功后再同步到本地
        let ok = Ua::update_ua(ua, id)?;
        Ok(AppResponse::success(None, Some(ok)))
    }

    /// 删除ua信息
    pub fn delete_ua_handle(id: i8) -> Result<AppResponse<bool>> {
        // TODO: 先删除服务器中的，等服务器成功后再同步到本地
        let ok = Ua::delete_ua(id)?;
        Ok(AppResponse::success(None, Some(ok)))
    }

    /// 添加ua信息
    pub fn add_ua_handle(ua: Ua) -> Result<AppResponse<bool>> {
        // TODO: 先添加到服务器，等服务器成功后再同步到本地
        let ok = Ua::insert_ua(ua)?;
        Ok(AppResponse::success(None, Some(ok)))
    }
}

pub mod gpu {
    use models::gpu::Gpu;

    use super::*;

    /// 查询gpu信息
    pub fn list_gpu_handle() -> Result<AppResponse<Vec<Gpu>>> {
        // TODO: 判断本地数据库是否存在，本地没有再尝试获取服务器
        let gpus = Gpu::query_gpu()?;
        Ok(AppResponse::success(None, Some(gpus)))
    }

    /// 更新gpu信息
    pub fn update_gpu_handle(gpu: Gpu, id: i8) -> Result<AppResponse<bool>> {
        // TODO: 先更新到服务器，等服务器成功后再同步到本地
        let ok = Gpu::update_gpu(gpu, id)?;
        Ok(AppResponse::success(None, Some(ok)))
    }

    /// 删除gpu信息
    pub fn delete_gpu_handle(id: i8) -> Result<AppResponse<bool>> {
        // TODO: 先删除服务器中的，等服务器成功后再同步到本地
        let ok = Gpu::delete_gpu(id)?;
        Ok(AppResponse::success(None, Some(ok)))
    }

    /// 添加gpu信息
    pub fn add_gpu_handle(gpu: Gpu) -> Result<AppResponse<bool>> {
        // TODO: 先添加到服务器，等服务器成功后再同步到本地
        let ok = Gpu::insert_gpu(gpu)?;
        Ok(AppResponse::success(None, Some(ok)))
    }
}

pub mod cookie {
    use models::cookie::PluginsCookie;

    use super::*;

    /// 查询cookie信息
    pub fn list_gpu_handle(path: &str) -> Result<AppResponse<Vec<PluginsCookie>>> {
        // TODO: 判断本地数据库是否存在，本地没有再尝试获取服务器
        let cookies = PluginsCookie::query_cookie(path)?;
        Ok(AppResponse::success(None, Some(cookies)))
    }

    /// 更新cookie信息
    pub fn update_cookie_handle(cookie: PluginsCookie, path: &str) -> Result<AppResponse<bool>> {
        // TODO: 先更新到服务器，等服务器成功后再同步到本地
        let ok = PluginsCookie::update_cookie(cookie, path)?;
        Ok(AppResponse::success(None, Some(ok)))
    }

    /// 删除cookie信息
    pub fn delete_cookie_handle(path: &str, creation: &str) -> Result<AppResponse<bool>> {
        // TODO: 先删除服务器中的，等服务器成功后再同步到本地
        let ok = PluginsCookie::delete_cookie(path, creation)?;
        Ok(AppResponse::success(None, Some(ok)))
    }

    /// 添加cookie信息
    pub fn add_cookie_handle(
        cookie_path: &str,
        cookie: PluginsCookie,
    ) -> Result<AppResponse<bool>> {
        // TODO: 先添加到服务器，等服务器成功后再同步到本地
        let ok = PluginsCookie::insert_cookie(cookie_path, cookie)?;
        Ok(AppResponse::success(None, Some(ok)))
    }
}

/// 操作浏览器的mod
pub mod browser {
    use std::{collections::HashMap, process::ExitStatus};

    use super::*;
    use crate::{
        models::enviroment,
        utils::{
            command::{BrowserChildInfo, Processer},
            common::get_debug_port,
        },
    };

    // get_proxy_from_registry
    pub async fn start(id: i8, processer: &mut Processer) -> Result<AppResponse<bool>> {
        if let Some(browser) = enviroment::Browser::query_browser_by_id(id)? {
            let port = get_debug_port().await?;
            let browser_child_info = BrowserChildInfo::new(browser, port, "");

            let ok = processer.start_browser(browser_child_info).await?;
            Ok(AppResponse::success(
                None,
                Some(ok.data.unwrap_or_default()),
            ))
        } else {
            Ok(AppResponse::success(None, Some(false)))
        }
    }

    pub async fn stop(id: i8, processer: &mut Processer) -> Result<AppResponse<ExitStatus>> {
        processer.stop_browser(id).await
    }

    pub async fn is_active(id: i8, processer: &mut Processer) -> Result<AppResponse<bool>> {
        processer.status(id).await
    }

    pub async fn view_active(processer: &mut Processer) -> Result<AppResponse<HashMap<i8, bool>>> {
        processer.all_status().await
    }
}

pub async fn delete_cache() -> Result<AppResponse<()>> {
    let data_cache_location = &AConfig.setting.data_location;
    let user_data_cache_location = &AConfig.setting.user_data_location;

    delete_data_file(&data_cache_location, "").await?;
    delete_data_file(&user_data_cache_location, "").await?;

    Ok(AppResponse::success(None, None))
}
