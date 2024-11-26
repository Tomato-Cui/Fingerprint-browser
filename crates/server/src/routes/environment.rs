use crate::response::AppResponse;
use axum::{extract::Json, response::IntoResponse};
use axum::{routing::post, Router};
use serde::{Deserialize, Serialize};

pub fn build_environment_router() -> Router {
    Router::new().nest(
        "/user/environment",
        Router::new()
            .route("/create", post(create_environment::handle))
            .route("/update", post(update_environment::handle))
            .route("/list", post(list_environment::handle))
            .route("/delete", post(delete_environment::handle))
            .route("/regroup", post(regroup_environment::handle))
            .route("/delete-cache", post(delete_cache_environment::handle)),
    )
}

mod create_environment {
    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Param {
        pub name: Option<String>,        // 对该环境进行命名，限制100字，方便记忆管理
        pub domain_name: Option<String>, // 账号平台的域名：facebook.com, amazon.com...会在打开浏览器时默认访问
        pub open_urls: Option<Vec<String>>, // 浏览器打开时访问的其他url地址，不填则默认只打开domain_name的地址
        pub repeat_config: Option<Vec<i32>>, // 账号去重，默认允许重复，支持0允许重复；2根据账密去重；3根据cookie去重；4根据c_user去重(c_user是FaceBook专有标记)
        pub username: Option<String>,        // 账号密码或者Cookie至少填一个；账号允许重复则都可不填
        pub password: Option<String>,        // 账号密码或者Cookie至少填一个；账号允许重复则都可不填
        pub fakey: Option<String>, // 填写2FA密钥。适用于网站的二次验证码生成，类似Google身份验证器
        pub cookie: Option<String>, // 账号密码或者Cookie至少填一个；账号允许重复则都可不填；支持JSON和Netscape格式
        pub ignore_cookie_error: Option<i32>, // 0：校验cookie失败时，直接返回cookie格式不正确；1：校验cookie失败时，过滤掉格式错误的数据，保留正确格式的cookie，仅支持Netscape格式
        pub group_id: String,                 // 添加到对应分组的ID，未分配分组则可以传0
        pub ip: Option<String>,               // 环境使用的代理IP，代理软件为lumauto、oxylabs填写
        pub country: Option<String>, // 环境使用的代理国家/地区，lumauto、oxylabs如果没有IP则需要填写国家
        pub region: Option<String>,  // 环境使用的代理州/省，可不填
        pub city: Option<String>,    // 环境使用的代理城市，可不填
        pub remark: Option<String>,  // 备注
        pub ipchecker: Option<String>, // IP查询渠道，支持传入ip2location、ipapi
        pub sys_app_cate_id: Option<i32>, // 可传入应用分类ID，0为跟随团队应用
        pub user_proxy_config: Option<String>, // 环境代理配置，具体查看参数对象userProxyConfig
        pub proxyid: Option<String>, // 可传入代理id或random（随机一个代理）
        pub fingerprint_config: String, // 指纹配置，具体查看参数对象fingerprintConfig
    }

    /// 新建浏览器
    ///
    /// 新建浏览器，支持配置平台账密和Cookie、代理ID和代理信息、指纹信息等等，新建成功后返回浏览器的环境ID。
    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        AppResponse::success(None, Some(payload))
    }
}

mod update_environment {
    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Param {
        pub user_id: String,                    // 需要修改的环境ID（必填）
        pub name: Option<String>,               // 环境名称，限制100字，方便记忆管理（可选）
        pub domain_name: Option<String>, // 账号平台的域名：facebook.com, amazon.com...会在打开浏览器时默认访问（可选）
        pub open_urls: Option<Vec<String>>, // 浏览器打开时访问的其他url地址，不填则默认只打开domain_name的地址（可选）
        pub username: Option<String>, // 账号密码或者Cookie至少填一个；账号允许重复则都可不填（可选）
        pub password: Option<String>, // 账号密码或者Cookie至少填一个；账号允许重复则都可不填（可选）
        pub fakey: Option<String>, // 填写2FA密钥。适用于网站的二次验证码生成，类似Google身份验证器（可选）
        pub cookie: Option<String>, // 账号密码或者Cookie至少填一个；账号允许重复则都可不填；支持JSON和Netscape格式（可选）
        pub ignore_cookie_error: Option<i32>, // 0：校验cookie失败时，直接返回cookie格式不正确；1：校验cookie失败时，过滤掉格式错误的数据，保留正确格式的cookie，仅支持Netscape格式（可选）
        pub ip: Option<String>, // 环境使用的代理IP，代理软件为lumauto、oxylabs填写（可选）
        pub country: Option<String>, // 环境使用的代理国家/地区，lumauto、oxylabs如果没有IP则需要填写国家（可选）
        pub region: Option<String>,  // 环境使用的代理州/省，可不填（可选）
        pub city: Option<String>,    // 环境使用的代理城市，可不填（可选）
        pub remark: Option<String>,  // 备注（可选）
        pub sys_app_cate_id: Option<String>, // 可传入应用分类ID，0为跟随团队应用（可选）
        pub user_proxy_config: Option<String>, // 代理配置，具体查看参数对象userProxyConfig（可选）
        pub proxyid: Option<String>, // 可传入代理id或random（随机一个代理）（可选）
        pub fingerprint_config: Option<String>, // 指纹配置，具体查看参数对象fingerprintConfig（必填）
    }

    /// 新建浏览器
    ///
    /// 新建浏览器，支持配置平台账密和Cookie、代理ID和代理信息、指纹信息等等，新建成功后返回浏览器的环境ID。
    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        AppResponse::success(None, Some(payload))
    }
}

mod list_environment {
    use std::collections::HashMap;

    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Param {
        pub group_id: Option<String>, // 指定分组ID查询环境，默认不传递则查询所有分组的环境（可选）
        pub user_id: Option<String>,  // 指定环境ID查询（可选）
        pub serial_number: Option<String>, // 指定环境编号查询（可选）
        pub user_sort: Option<HashMap<String, String>>, // 查询环境返回的结果可以按指定类型排序，支持serial_number、last_open_time、created_time，asc和desc两个值（可选）
        pub page: Option<u32>,                          // 页码，默认1，数量多需要翻页时用（可选）
        pub page_size: Option<u32>,                     // 每页大小，默认每页1 ，最大100（可选）
    }

    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        AppResponse::success(None, Some(payload))
    }
}

mod delete_environment {
    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Param {
        pub user_ids: Vec<String>, // 需要删除的环境ID，数组格式
    }

    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        AppResponse::success(None, Some(payload))
    }
}

mod regroup_environment {
    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Param {
        pub user_ids: Vec<String>, // 需要分组的环境ID，数组格式
        pub group_id: String,      // 对应的分组ID
    }

    pub async fn handle(Json(payload): Json<Param>) -> AppResponse<Param> {
        AppResponse::<Param>::success(None, Some(payload))
    }
}

mod delete_cache_environment {
    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Param {}

    pub async fn handle(Json(_payload): Json<Param>) -> impl IntoResponse {
        AppResponse::<()>::success(None, None)
    }
}
