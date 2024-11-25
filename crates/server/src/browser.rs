use axum::{extract::Json, response::IntoResponse};
use axum::{routing::post, Router};
use serde::{Deserialize, Serialize};

pub fn build_browser_router() -> Router {
    Router::new().nest(
        "/browser",
        Router::new()
            .route("/start", post(start_browser::handle)) // 浏览器启动
            .route("/stop", post(stop_browser::handle)) // 关闭浏览器
            .route("/active", post(active_browser::handle)) // 检查浏览器状态
            .route("/view-active", post(view_active_browser::handle)), // 查询以启动的浏览器状态
    )
}

mod start_browser {
    use super::*;

    #[derive(Deserialize, Serialize)]
    pub struct Param {
        pub user_id: String,
        pub serial_number: String,
        pub open_tabs: bool,
        pub ip_tab: bool,
        pub new_first_tab: bool,
        pub launch_args: Vec<String>,
        pub headless: bool,
        pub disable_password_filling: String,
        pub clear_cache_after_closing: String,
        pub enable_password_saving: String,
        pub cdp_mask: String,
    }

    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        todo!()
    }
}

mod stop_browser {
    use super::*;

    #[derive(Deserialize, Serialize)]
    pub struct Param {}

    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        todo!()
    }
}

mod active_browser {
    use super::*;

    #[derive(Deserialize, Serialize)]
    pub struct Param {}

    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        todo!()
    }
}

mod view_active_browser {
    use super::*;

    #[derive(Deserialize, Serialize)]
    pub struct ViewActiveBrowserRequestParam {}

    pub async fn handle(Json(payload): Json<ViewActiveBrowserRequestParam>) -> impl IntoResponse {
        todo!()
    }
}
