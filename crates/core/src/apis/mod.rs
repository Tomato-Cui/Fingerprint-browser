use crate::{
    errors::ApplicationServerError,
    models::{self, browser::Browser},
    utils::response::AppResponse,
};

pub type Result<T> = core::result::Result<T, ApplicationServerError>;

pub mod browser {
    use super::*;

    /// 获取浏览器列表
    pub fn get_browser_list_handle() -> Result<AppResponse<Vec<Browser>>> {
        let browsers = models::browser::Browser::query_browser()?;
        // TODO: 判断本地数据库是否存在，本地没有再尝试获取服务器
        Ok(AppResponse::success(None, Some(browsers)))
    }

    /// 更新浏览器环境
    pub fn update_browser_handle(browser: Browser, id: i8) -> Result<AppResponse<bool>> {
        // TODO: 先更新到服务器，等服务器成功后再同步到本地
        // let browser_info: Browser = serde_json::from_str(browser)?;
        let ok = models::browser::Browser::update_browser(browser, id)?;
        Ok(AppResponse::success(None, Some(ok)))
    }

    /// 删除浏览器环境
    pub fn delete_browser_handle(id: i8) -> Result<AppResponse<bool>> {
        // TODO: 先删除服务器中的，等服务器成功后再同步到本地
        let ok = models::browser::Browser::delete_browser(id)?;
        Ok(AppResponse::success(None, Some(ok)))
    }

    /// 添加浏览器环境
    pub fn add_browser_handle(browser: Browser) -> Result<AppResponse<bool>> {
        // TODO: 先添加到服务器，等服务器成功后再同步到本地
        // let browser_info: Browser = serde_json::from_str(browser)?;
        let ok = models::browser::Browser::insert_browser(browser)?;
        Ok(AppResponse::success(None, Some(ok)))
    }

    /// 更新浏览器状态
    pub fn update_browser_status_handle(id: i8, status: bool) -> Result<AppResponse<bool>> {
        let ok = models::browser::Browser::update_browser_status(id, status)?;
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
