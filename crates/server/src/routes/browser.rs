use axum::{extract::Json, response::IntoResponse};
use axum::{routing::post, Router};
use cores::apis::browser;
use serde::{Deserialize, Serialize};

pub fn build_browser_router() -> Router {
    Router::new().nest(
        "/browser",
        Router::new()
            .route("/start", post(starts_browser::handle)) // 浏览器启动
            .route("/stop", post(stop_browser::handle)) // 关闭浏览器
            .route("/statu", post(active_browser::handle)) // 检查浏览器状态
            .route("/status", post(view_active_browser::handle)), // 查询以启动的浏览器状态
    )
}

mod starts_browser {
    use browser::StartEnvironmentParams;

    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Param {
        pub browsers: Vec<StartEnvironmentParams>, // 环境ID，创建环境成功后生成的唯一ID。
                                                   // pub serial_number: Option<String>,    // 通过环境编号关闭，如果已传递环境ID则优先用环境ID。
                                                   // pub open_tabs: Option<bool>, // 是否打开平台和历史页面. true: (默认)打开,false不打开
                                                   // pub ip_tab: Option<bool>,    // 是否打开ip检测页，true: (默认)打开，false: 不打开。
                                                   // pub new_first_tab: Option<bool>, // 是否使用新版ip检测页： true: （默认）新版，false：旧版。
                                                   // pub launch_args: Option<Vec<String>>, // 启动参数，例：--blink-settings=imagesEnabled=false: 禁止图片加载 --disable-notifications: 禁用通知。 使用API时，如果API传了“launch_args”，则以API传的值为准。
                                                   // pub headless: Option<bool>, // 是否启动headless浏览器(无头浏览器) true: 是，false:否（默认）。
                                                   // pub disable_password_filling: Option<bool>, // 是否禁用填充账密功能 true: 是，false: 否（默认）。
                                                   // pub clear_cache_after_closing: Option<bool>, // 关闭浏览器后是否清除缓存 true: 是，false:否（默认）。
                                                   // pub enable_password_saving: Option<bool>, // 是否允许保存密码 true:是，false: 否（默认）。
                                                   // pub cdp_mask: Option<bool>,               // 是否屏蔽 CDP 检测 true：是（默认），false：否。
    }

    /// 启动浏览器
    ///
    /// 用于启动浏览器，需要指定环境ID，启动成功后可以获取浏览器debug接口用于执行selenium和puppeteer自动化。 Selenium需要使用到对应内核版本匹配的Webdriver。需更新到应用版本3.4.1以上，启动浏览器后可在返回值中拿到对应的Webdriver的路径。
    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        browser::starts(payload.browsers)
            .await
            .unwrap_or_else(|e| e.into())
    }
}

mod stop_browser {
    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Param {
        pub browser_ids: Vec<i32>, // 环境ID，创建环境成功后生成的唯一ID。
                                  // pub serial_number: Option<String>, // 通过环境编号关闭，如果已传递环境ID则优先用环境ID。
    }

    /// 关闭浏览器
    ///
    /// 用于关闭对应的浏览器，需要指定环境ID。
    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        browser::stop(payload.browser_ids)
            .await
            .unwrap_or_else(|e| e.into())
    }
}

mod active_browser {
    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Param {
        pub browser_id: i32, // 环境ID，创建环境成功后生成的唯一ID。
                            // pub serial_number: Option<String>, // 通过环境编号关闭，如果已传递环境ID则优先用环境ID。
    }

    /// 检查启动状态
    ///
    /// 用于检查浏览器的启动状态，需要指定环境ID。
    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        browser::is_active(payload.browser_id)
            .await
            .unwrap_or_else(|e| e.into())
    }
}

mod view_active_browser {

    use super::*;

    // #[derive(Deserialize, Serialize, Debug)]
    // pub struct ViewActiveBrowserRequestParam {}

    /// 查询已启动浏览器
    ///
    /// 查询当前设备所有已打开的浏览器。
    pub async fn handle() -> impl IntoResponse {
        browser::view_active().await.unwrap_or_else(|e| e.into())
    }
}
