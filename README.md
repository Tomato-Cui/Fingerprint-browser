### HUBSTUDIO & ADSPOWER BROWSER

hubstudio & adspower browser

### 目录结构介绍

```
├── Cargo.lock
├── Cargo.toml
├── README.md         # reade me
├── clients              # 客户端可执行文件
├── config.toml       # 配置文件
├── crates
│   ├── cores         # 核心
│   ├── server        # api服务
│   └── views         # tauri
├── front             # 前端代码
│   ├── README.md
│   ├── dist
│   ├── env.d.ts
│   ├── index.html
│   ├── node_modules
│   ├── package-lock.json
│   ├── package.json
│   ├── pnpm-lock.yaml
│   ├── public
│   ├── src
│   ├── text.txt
│   ├── tsconfig.app.json
│   ├── tsconfig.json
│   ├── tsconfig.node.json
│   └── vite.config.ts
├── migrations        # 数据库迁移文件
└── tests             # 测试代码
```

### 程序执行

```sh
git clone -b client https://github.com/Tu-guang/zonghu-fingerprint-frontend.git

# 执行前端代码
cd zonghu-fingerprint-frontend/frontend && npm i && npm run dev

# 执行客户端代码
cd zonghu-fingerprint-frontend/clients && cargo run --bin clients
```

### 代码块一
```
use crate::handles::team;
use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn build_router() -> Router {
    Router::new().nest(
        "/teams",
        Router::new()
            .route("/", get(team::query))
            .route("/id", get(team::query_by_id))
            .route("/is-leader", get(team::is_leader))
            .route("/search-by-name", get(team::query_search_by_name))
            .route("/current", get(team::query_current_team))
            .route("/all-user", post(team::query_team_all_user))
            .route("/all-blocked-user", post(team::query_team_all_blocked_user))
            .route("/group/all-user", post(team::query_team_group_all_user))
            .route("/blocked", put(team::blocked))
            .route("/un-blocked", put(team::unblocked))
            .route("/create", post(team::create))
            .route("/", put(team::modify))
            .route("/switch", put(team::switch_team))
            .route("/remove-user", put(team::remove_current_user))
            .route("/user-info", put(team::modify_team_user_info))
            .route("/", delete(team::delete)),
    )
}

use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Extension, Json,
};

use crate::{
    entities::{team::*, IdPayload},
    middlewares::CurrentUser,
    response::AppResponse,
};

use super::{success_message, warn_message};

pub async fn query_by_id(Json(payload): Json<IdPayload>) -> impl IntoResponse {
    match services::team::query_by_id(payload.id).await {
        Ok(data) => AppResponse::success(success_message("查询成功"), Some(data)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn is_leader(
    state: Extension<CurrentUser>,
    Path(team_id): Path<u32>,
) -> impl IntoResponse {
    match services::team::is_leader(&state.user_uuid, team_id).await {
        Ok(data) => AppResponse::success(success_message("查询成功"), Some(data)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn query_search_by_name(Path(team_name): Path<String>) -> impl IntoResponse {
    match services::team::query_by_search_name(&team_name).await {
        Ok(data) => AppResponse::success(success_message("查询成功"), Some(data)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn query(state: Extension<CurrentUser>, payload: Query<Pagination>) -> impl IntoResponse {
    match services::team::query(&state.user_uuid, payload.page_num, payload.page_size).await {
        Ok(data) => AppResponse::success(success_message("查询成功"), Some(data)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn query_current_team(state: Extension<CurrentUser>) -> impl IntoResponse {
    match services::team::query_current_team_info(&state.user_uuid).await {
        Ok(data) => AppResponse::success(success_message("查询成功"), Some(data)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn query_team_all_user(
    state: Extension<CurrentUser>,
    Json(payload): Json<TeamQueryPayload>,
) -> impl IntoResponse {
    match services::team::query_team_all_user(
        &state.user_uuid,
        payload.team_id,
        payload.page_num,
        payload.page_size,
    )
    .await
    {
        Ok(data) => AppResponse::success(success_message("查询成功"), Some(data)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn query_team_all_blocked_user(
    state: Extension<CurrentUser>,
    payload: Json<TeamQueryPayload>,
) -> impl IntoResponse {
    match services::team::query_team_all_blocked_user(
        &state.user_uuid,
        payload.team_id,
        payload.page_num,
        payload.page_size,
    )
    .await
    {
        Ok(data) => AppResponse::success(success_message("查询成功"), Some(data)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn query_team_group_all_user(
    state: Extension<CurrentUser>,
    payload: Json<TeamGroupQueryPayload>,
) -> impl IntoResponse {
    match services::team::query_team_group_all_user(
        &state.user_uuid,
        payload.team_id,
        payload.team_group_id,
        payload.page_num,
        payload.page_size,
    )
    .await
    {
        Ok(data) => AppResponse::success(success_message("查询成功"), Some(data)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn blocked(
    state: Extension<CurrentUser>,
    payload: Json<BlockedPayload>,
) -> impl IntoResponse {
    match services::team::blocked(
        &state.user_uuid,
        &payload.current_user_uuid,
        payload.team_id,
    )
    .await
    {
        Ok(data) => AppResponse::success(success_message("拉黑成功"), Some(data)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn unblocked(
    state: Extension<CurrentUser>,
    payload: Json<BlockedPayload>,
) -> impl IntoResponse {
    match services::team::un_blocked(
        &state.user_uuid,
        &payload.current_user_uuid,
        payload.team_id,
    )
    .await
    {
        Ok(data) => AppResponse::success(success_message("取消拉黑成功"), Some(data)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn create(
    state: Extension<CurrentUser>,
    payload: Json<CreateTeamPayload>,
) -> impl IntoResponse {
    let team = models::team::Team {
        name: payload.name.clone(),
        description: Some(payload.description.clone()),
        ..Default::default()
    };

    match services::team::create(&state.user_uuid, &team).await {
        Ok(_) => AppResponse::success(success_message("创建成功"), Some(())),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn modify(payload: Json<ModifyTeamPayload>) -> impl IntoResponse {
    let id = payload.id as u32;
    let team = models::team::Team {
        id: id as i32,
        name: payload.name.clone(),
        description: Some(payload.description.clone()),
        ..Default::default()
    };

    match services::team::modify(id, &team).await {
        Ok(_) => AppResponse::success(success_message("更新成功"), Some(())),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn switch_team(state: Extension<CurrentUser>, Path(id): Path<u32>) -> impl IntoResponse {
    match services::team::switch_team(&state.user_uuid, id).await {
        Ok(_) => AppResponse::success(success_message("切换成功"), Some(())),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn remove_current_user(
    state: Extension<CurrentUser>,
    payload: Json<BlockedPayload>,
) -> impl IntoResponse {
    match services::team::remove_user(
        &state.user_uuid,
        payload.team_id,
        &payload.current_user_uuid,
    )
    .await
    {
        Ok(_) => AppResponse::success(success_message("移除成功"), Some(true)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn modify_team_user_info(
    state: Extension<CurrentUser>,
    payload: Json<ModifyTeamUserInfoPayload>,
) -> impl IntoResponse {
    match services::team::update_team_user_info(
        &state.user_uuid,
        payload.team_id,
        payload.description.clone(),
        payload.team_group_id,
        &payload.current_user_uuid,
    )
    .await
    {
        Ok(_) => AppResponse::success(success_message("更新成功"), Some(true)),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}

pub async fn delete(Json(payload): Json<IdPayload>) -> impl IntoResponse {
    match services::team::delete(payload.id).await {
        Ok(_) => AppResponse::success(success_message("删除成功"), Some(())),
        Err(e) => AppResponse::fail(warn_message(e)),
    }
}


use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    pub page_num: u32,
    pub page_size: u32,
}

#[derive(Deserialize)]
pub struct TeamQueryPayload {
    pub team_id: u32,
    pub page_num: u32,
    pub page_size: u32,
}

#[derive(Deserialize)]
pub struct TeamGroupQueryPayload {
    pub team_id: u32,
    pub team_group_id: u32,
    pub page_num: u32,
    pub page_size: u32,
}

#[derive(Deserialize)]
pub struct BlockedPayload {
    pub current_user_uuid: String,
    pub team_id: u32,
}

#[derive(Deserialize)]
pub struct CreateTeamPayload {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct ModifyTeamPayload {
    pub id: u32,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct ModifyTeamUserInfoPayload {
    pub team_id: u32,
    pub description: Option<String>,
    pub team_group_id: u32,
    pub current_user_uuid: String,
}
```

### 代码块二

#### 所有的接口
```
use crate::handles::user;
use axum::{
    routing::{get, post},
    Router,
};

pub fn build_router() -> Router {
    Router::new().nest(
        "/users",
        Router::new()
            .route("/login", post(user::handle_login))
            .route("/logout", get(user::handle_logout))
            .route("/register", post(user::handle_register))
            .route("/search-by-email", post(user::handle_search_by_email))
            .route("/reset-password", post(user::handle_reset_password))
            .route("/register/send", post(user::handle_register_send)),
    )
}
```

#### handles 

```
use crate::entities::user::*;
use crate::response::AppResponse;
use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn handle_login(Json(payload): Json<LoginPayload>) -> impl IntoResponse {
    match services::user::login(&payload.username, &payload.password).await {
        Ok(token) => AppResponse::success(
            Some("登录成功".to_string()),
            Some(json!({ "token": token })),
        ),
        Err(e) => AppResponse::fail(Some(format!("登录失败: {}", e))),
    }
}

pub async fn handle_logout() -> impl IntoResponse {
    match services::user::logout().await {
        Ok(_) => AppResponse::<()>::success(Some("退出成功".to_string()), Some(())),
        Err(e) => AppResponse::fail(Some(format!("退出失败: {}", e))),
    }
}

pub async fn handle_register(Json(payload): Json<RegisterPayload>) -> impl IntoResponse {
    match services::user::regsiter(
        &payload.email,
        &payload.code,
        &payload.username,
        &payload.password,
    )
    .await
    {
        Ok(_) => AppResponse::success(Some("注册成功".to_string()), Some(())),
        Err(e) => AppResponse::fail(Some(format!("注册失败: {}", e))),
    }
}

pub async fn handle_search_by_email(
    Json(payload): Json<SearchByEmailPayload>,
) -> impl IntoResponse {
    match services::user::query_search_by_email(&payload.email).await {
        Ok(data) => AppResponse::success(Some("搜索成功".to_string()), Some(data)),
        Err(e) => AppResponse::fail(Some(format!("搜索失败: {}", e))),
    }
}

pub async fn handle_reset_password(Json(payload): Json<ResetPasswordPayload>) -> impl IntoResponse {
    match services::user::reset_password(&payload.email, &payload.password1, &payload.password2)
        .await
    {
        Ok(_) => AppResponse::success(Some("重置密码成功".to_string()), Some(())),
        Err(e) => AppResponse::fail(Some(format!("重置密码失败: {}", e))),
    }
}

pub async fn handle_register_send(Json(payload): Json<RegisterSendPayload>) -> impl IntoResponse {
    match services::user::register_send(&payload.email).await {
        Ok(_) => AppResponse::success(Some("发送成功".to_string()), Some(true)),
        Err(e) => AppResponse::fail(Some(format!("发送失败: {}", e))),
    }
}
```

#### 请求参数
```
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterPayload {
    pub email: String,
    pub code: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ResetPasswordPayload {
    pub email: String,
    pub password1: String,
    pub password2: String,
}

#[derive(Deserialize)]
pub struct RegisterSendPayload {
    pub email: String,
}

#[derive(Deserialize)]
pub struct SearchByEmailPayload {
    pub email: String,
}
```

### 根据代码块二的生成示例
```
use cores::request::{client, JsonRespnse};
use serde_json::json;

pub async fn logout() -> Result<JsonRespnse, anyhow::Error> {
    let json_response: JsonRespnse = client::REQUEST
        .get(client::Client::build_url("/users/logout")?)
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("退出失败: {:?}", e))?;

    Ok(json_response)
}

pub async fn login(nickname: &str, password: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
            "username": nickname,
            "password": password
    });

    let response = client::REQUEST
        .post(client::Client::build_url("/users/login")?, &data)
        .await?;

    let json_response = response
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("登录失败: {}", e))?;

    Ok(json_response)
}

pub async fn regsiter(
    email: &str,
    code: &str,
    nickname: &str,
    password: &str,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "email": email,
        "code": code,
        "username": nickname,
        "password": password,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/users/register")?, &data)
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("注册失败: {}", e))?;

    Ok(json_response)
}

pub async fn query_search_by_email(email: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "email": email,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/users/search-by-email")?, &data)
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("查询失败: {}", e))?;

    Ok(json_response)
}

pub async fn reset_password(
    email: &str,
    password1: &str,
    password2: &str,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "email": email,
        "password1": password1,
        "password2": password2,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/users/reset-password")?, &data)
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("重置失败: {}", e))?;

    Ok(json_response)
}

pub async fn register_send(email: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "email": email,
    });

    let json_response = client::REQUEST
        .post(client::Client::build_url("/users/register-send")?, &data)
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("重置失败: {}", e))?;

    Ok(json_response)
}
```

参照代码块二，我已经提供了生成示例，请根据代码块一生成相似的示例内容(忽略state:Extesion<CurrentUser>, 这个已经包含在核心代码中了。)。