use axum::{extract::Json, response::IntoResponse};
use axum::{routing::post, Router};
use cores::apis::group;
use serde::{Deserialize, Serialize};

pub fn build_group_router() -> Router {
    Router::new().nest(
        "/group",
        Router::new()
            .route("/create", post(create_group::handle))
            .route("/update", post(update_group::handle))
            .route("/list", post(list_group::handle)),
    )
}

mod create_group {

    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Param {
        pub group_name: String, // group_name	text	是	-	group1	添加分组的名称，名称不能重复。
        pub group_description: Option<String>, // remark	text	否	-	123	添加分组的备注(需升级到v2.6.7.2及以上)。
    }

    /// 创建分组
    ///
    /// 建环境分组，用于创建环境时将环境进行分组，名称不能重复。创建成功后将返回group_id分组ID，分组ID为0是系统创建的默认分组。
    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        group::add_group_handle(&payload.group_name, payload.group_description)
            .await
            .unwrap_or_else(|e| e.into())
    }
}

mod update_group {
    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Param {
        pub group_id: i8,       // group_id	    text	是	-	123	需要修改的分组ID。
        pub group_name: String, // group_name	text	是	-	group1	添加分组的名称，名称不能重复。
        pub group_description: Option<String>, // remark	    text	否	-	123	添加分组的备注(需升级到v2.6.7.2及以上)。
    }

    /// 修改分组
    ///
    /// 修改分组信息，可以修改分组名称，名称不能重复。
    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        group::update_group_handle(
            &payload.group_name,
            payload.group_description,
            payload.group_id,
        )
        .await
        .unwrap_or_else(|e| e.into())
    }
}

mod list_group {
    use cores::apis::PageParam;

    use super::*;

    // #[derive(Deserialize, Serialize, Debug)]
    // pub struct Param {
    // pub group_name: Option<String>, // group_name	text	是	-	group1	添加分组的名称，名称不能重复。
    // pub page: Option<i8>,      // page	否	1	1	页码默认1第一页。
    // pub page_size: Option<i8>, // page_size	否	1	10	每页条数默认1条，最大2000。
    // }

    /// 查询分组
    ///
    /// 查询分组信息，分组信息包括分组ID和分组名称，其中分组ID用于创建环境时将环境进行分组，分组ID为0是系统创建的默认分组。
    pub async fn handle(Json(payload): Json<PageParam>) -> impl IntoResponse {
        group::list_group_handle(payload)
            .await
            .unwrap_or_else(|e| e.into())
    }
}
