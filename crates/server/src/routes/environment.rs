use axum::{extract::Json, response::IntoResponse};
use axum::{routing::get, routing::post, Router};
use cores::{apis, models::enviroment::Environment};
use serde::{Deserialize, Serialize};

pub fn build_environment_router() -> Router {
    Router::new().nest(
        "/user/environment",
        Router::new()
            .route("/create", post(create_environment::handle))
            .route("/update", post(update_environment::handle))
            .route("/list", post(list_environment::handle))
            .route("/list-by-group", post(list_environment_by_group::handle))
            .route("/:id", get(find_by_id_environment::handle))
            .route("/delete", post(delete_environment::handle))
            .route("/regroup", post(regroup_environment::handle))
            .route("/delete-cache", get(delete_cache_environment::handle)),
    )
}

mod create_environment {

    use super::*;

    /// 新建环境
    pub async fn handle(Json(payload): Json<Environment>) -> impl IntoResponse {
        apis::enviroment::add_browser_handle(payload)
            .await
            .unwrap_or_else(|e| e.into())
    }
}

mod update_environment {

    use super::*;

    /// 更新环境
    pub async fn handle(Json(payload): Json<Environment>) -> impl IntoResponse {
        apis::enviroment::update_browser_handle(payload)
            .await
            .unwrap_or_else(|e| e.into())
    }
}

mod list_environment {
    use super::*;
    use apis::PageParam;

    #[axum::debug_handler]
    pub async fn handle(Json(payload): Json<PageParam>) -> impl IntoResponse {
        apis::enviroment::get_browser_list_handle(payload)
            .await
            .unwrap_or_else(|e| e.into())
    }
}

mod find_by_id_environment {
    use axum::extract::Path;

    use super::*;
    #[axum::debug_handler]
    pub async fn handle(Path(id): Path<i32>) -> impl IntoResponse {
        apis::enviroment::get_browser_by_id_handle(id)
            .await
            .unwrap_or_else(|e| e.into())
    }
}

mod list_environment_by_group {
    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Param {
        pub group_id: i32,        // 每页大小，默认每页1 ，最大100（可选）
        pub page_num: Option<i32>,  // 页码，默认1，数量多需要翻页时用（可选）
        pub page_size: Option<i32>, // 每页大小，默认每页1 ，最大100（可选）
    }

    #[axum::debug_handler]
    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        apis::enviroment::get_browser_list_by_group_handle(
            payload.group_id,
            payload.page_num,
            payload.page_size,
        )
        .await
        .unwrap_or_else(|e| e.into())
    }
}

mod delete_environment {
    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Param {
        pub browser_id: i32, // 需要删除的环境ID，数组格式
    }

    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        apis::enviroment::delete_browser_handle(payload.browser_id)
            .await
            .unwrap_or_else(|e| e.into())
    }
}

mod regroup_environment {
    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Param {
        pub browser_id: i32, // 需要分组的环境ID，数组格式
        pub group_id: i32,   // 对应的分组ID
    }

    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        apis::enviroment::update_browser_group_handle(payload.browser_id, payload.group_id)
            .await
            .unwrap_or_else(|e| e.into())
    }
}

mod delete_cache_environment {
    use super::*;

    // #[derive(Deserialize, Serialize, Debug)]
    // pub struct Param {}
    // pub async fn handle(Json(_payload): Json<Param>) -> impl IntoResponse {
    pub async fn handle() -> impl IntoResponse {
        apis::delete_cache().await.unwrap_or_else(|e| e.into())
    }
}
