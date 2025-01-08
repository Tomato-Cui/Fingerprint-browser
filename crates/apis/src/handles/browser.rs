use axum::response::IntoResponse;
use axum::Json;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::entities::browser::*;
use crate::response::AppResponse;

use super::{success_message, warn_message};

pub async fn start(Json(payload): Json<StartPayload>) -> impl IntoResponse {
    match services::command::start_browser(&payload.environment_uuid).await {
        Ok(data) => AppResponse::<Value>::success(success_message("开启成功"), Some(data)),
        Err(r) => AppResponse::<Value>::fail(warn_message(&format!("开启失败: {}", r))),
    }
}

pub async fn starts(Json(payload): Json<StartsPayload>) -> impl IntoResponse {
    let mut result = HashMap::new();

    for environment_uuid in payload.environment_uuids {
        match services::command::start_browser(&environment_uuid).await {
            Ok(v) => {
                result.insert(environment_uuid.to_string(), v);
            }
            Err(e) => {
                result.insert(
                    environment_uuid.to_string(),
                    json!({
                        "environment_id": environment_uuid.to_string(),
                        "status":  false,
                        "message": format!("启动失败: {}", e),
                    }),
                );
            }
        };
    }

    AppResponse::success(None, Some(result))
}

pub async fn stops(Json(payload): Json<StopsPayload>) -> impl IntoResponse {
    match services::command::stop(payload.environment_uuids).await {
        Ok(data) => {
            AppResponse::<HashMap<String, String>>::success(success_message("关闭成功"), Some(data))
        }
        Err(r) => {
            AppResponse::<HashMap<String, String>>::fail(warn_message(&format!("关闭失败: {}", r)))
        }
    }
}

pub async fn stop_all() -> impl IntoResponse {
    match services::command::stop_all().await {
        Ok(ok) => AppResponse::<bool>::success(success_message("关闭成功"), Some(ok)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("关闭成功: {}", r))),
    }
}

pub async fn status() -> impl IntoResponse {
    match services::command::view_active().await {
        Ok(data) => {
            AppResponse::<HashMap<String, bool>>::success(success_message("查询成功"), Some(data))
        }
        Err(r) => {
            AppResponse::<HashMap<String, bool>>::fail(warn_message(&format!("查询失败: {}", r)))
        }
    }
}
