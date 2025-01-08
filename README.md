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

#### 调用请求的方法
```
use cores::request::JsonRespnse;
use models::environment_account::EnvironmentAccount;
use services_remote::requests::environment_account;

use super::user::get_user_id;

#[tauri::command]
pub async fn environment_account_query_id(id: u32) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(environment_account::query_by_id(id).await?)
}

#[tauri::command]
pub async fn environment_account_query(
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(environment_account::query(page_num, page_size).await?)
}

#[tauri::command]
pub async fn environment_account_query_current(
    environmnet_uuid: &str,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;

    Ok(environment_account::query_current_by_current_environment(
        environmnet_uuid,
        page_num,
        page_size,
    )
    .await?)
}

#[tauri::command]
pub async fn environment_account_create(
    payload: EnvironmentAccount,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(environment_account::create(payload).await?)
}

#[tauri::command]
pub async fn environment_account_modify(
    payload: EnvironmentAccount,
) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(environment_account::modify(payload).await?)
}

#[tauri::command]
pub async fn environment_account_delete(id: u32) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;

    Ok(environment_account::delete(id).await?)
}

#[tauri::command]
pub async fn environment_account_batch_delete(ids: Vec<u32>) -> Result<JsonRespnse, tauri::Error> {
    let _ = get_user_id().await?;

    Ok(environment_account::batch_delete(ids).await?)
}
```

#### 请求的方法

```
use cores::request::{client, JsonRespnse};
use serde_json::json;

pub async fn query_by_id(id: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environment-accounts/query/id")?,
            &data,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("查询失败: {}", e))?;

    Ok(json_response)
}

pub async fn query(page_num: u32, page_size: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "page_num": page_num,
        "page_size": page_size,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environment-accounts/query")?,
            &data,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("查询失败: {}", e))?;

    Ok(json_response)
}

pub async fn query_current_by_current_environment(
    environmnet_uuid: &str,
    page_num: u32,
    page_size: u32,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environmnet_uuid": environmnet_uuid,
        "page_num": page_num,
        "page_size": page_size,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environment-accounts//query/environment/uuid")?,
            &data,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("查询失败: {}", e))?;

    Ok(json_response)
}

pub async fn create(
    account: models::environment_account::EnvironmentAccount,
) -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environment-accounts")?,
            &account,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("创建失败: {}", e))?;

    Ok(json_response)
}

pub async fn modify(
    account: models::environment_account::EnvironmentAccount,
) -> Result<JsonRespnse, anyhow::Error> {
    let json_response = client::REQUEST
        .put(
            client::Client::build_url("/environment-accounts")?,
            &account,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("更新失败: {}", e))?;

    Ok(json_response)
}

pub async fn delete(id: u32) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "id": id,
    });

    let json_response = client::REQUEST
        .delete(client::Client::build_url("/environment-accounts")?, &data)
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("删除失败: {}", e))?;

    Ok(json_response)
}

pub async fn batch_delete(ids: Vec<u32>) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "ids": ids,
    });

    let json_response = client::REQUEST
        .delete(
            client::Client::build_url("/environment-accounts/batch")?,
            &data,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("删除失败: {}", e))?;

    Ok(json_response)
}

```

### 代码块二

```
use cores::request::{client, JsonRespnse};
use serde_json::json;

pub async fn query_by_uuid(environment_uuid: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuid": environment_uuid,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environment-cookies/environment/id")?,
            &data,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("查询失败: {}", e))?;

    Ok(json_response)
}

pub async fn create(
    environment_uuid: &str,
    cookie_str: &str,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuid": environment_uuid,
        "cookie_str": cookie_str,
    });

    let json_response = client::REQUEST
        .post(
            client::Client::build_url("/environment-cookies/environment")?,
            &data,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("创建失败: {}", e))?;

    Ok(json_response)
}

pub async fn modify(
    environment_uuid: &str,
    cookie_str: &str,
) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuid": environment_uuid,
        "cookie_str": cookie_str,
    });

    let json_response = client::REQUEST
        .put(
            client::Client::build_url("/environment-cookies/environment")?,
            &data,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("更新失败: {}", e))?;

    Ok(json_response)
}

pub async fn delete(environment_uuid: &str) -> Result<JsonRespnse, anyhow::Error> {
    let data = json!({
        "environment_uuid": environment_uuid,
    });

    let json_response = client::REQUEST
        .delete(
            client::Client::build_url("/environment-cookies/environment")?,
            &data,
        )
        .await?
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("删除失败: {}", e))?;

    Ok(json_response)
}


```

```
use crate::response::AppResponse;
use models::environmnet_cookie::EnvironmentCookie;
use services_remote::requests::environment_cookie;

use super::user::get_user_id;

#[tauri::command]
pub async fn environment_cookie_query_environment_uuid(
    environment_uuid: String,
) -> Result<AppResponse<Vec<EnvironmentCookie>>, tauri::Error> {
    let _ = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    Ok(
        match services::environmnet_cookie::query(&environment_uuid).await {
            Ok(data) => AppResponse::<Vec<EnvironmentCookie>>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Vec<EnvironmentCookie>>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn environment_cookie_create(
    environment_uuid: String,
    cookie_str: String,
) -> Result<AppResponse<bool>, tauri::Error> {
    let _ = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
        Some(format!("创建失败: {}", v))
    });

    let cookie = EnvironmentCookie {
        value: cookie_str.to_string(),
        environment_uuid: Some(environment_uuid),
        ..Default::default()
    };

    Ok(match services::environmnet_cookie::create(&cookie).await {
        Ok(data) => {
            if data {
                AppResponse::<bool>::success(success_msg, Some(data))
            } else {
                AppResponse::<bool>::fail(warn_msg("未知错误".to_string()))
            }
        }
        Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
    })
}

#[tauri::command]
pub async fn environment_cookie_modify(
    environment_uuid: String,
    cookie_str: String,
) -> Result<AppResponse<bool>, tauri::Error> {
    let _ = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
        Some(format!("更新失败: {}", v))
    });

    let cookie = EnvironmentCookie {
        value: cookie_str.clone(),
        ..Default::default()
    };

    Ok(
        match services::environmnet_cookie::modify(&environment_uuid, &cookie).await {
            Ok(data) => {
                if data {
                    AppResponse::<bool>::success(success_msg, Some(data))
                } else {
                    AppResponse::<bool>::fail(warn_msg("未知错误".to_string()))
                }
            }
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn environment_cookie_delete(
    environmnet_uuid: &str,
) -> Result<AppResponse<bool>, tauri::Error> {
    let _ = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
        Some(format!("删除失败: {}", v))
    });

    Ok(
        match services::environmnet_cookie::delete(&environmnet_uuid).await {
            Ok(data) => {
                if data {
                    AppResponse::<bool>::success(success_msg, Some(data))
                } else {
                    AppResponse::<bool>::fail(warn_msg("未知错误".to_string()))
                }
            }
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        },
    )
}
```