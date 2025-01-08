use axum::response::IntoResponse;
use axum::Json;
use models::environmnet_cookie::EnvironmentCookie;

use crate::entities::environment_cookie::*;
use crate::response::AppResponse;

use super::{success_message, warn_message};

pub async fn query_by_uuid(
    Json(payload): Json<QueryByEnvironmentUuidPayload>,
) -> impl IntoResponse {
    match services::environmnet_cookie::query(&payload.environment_uuid).await {
        Ok(data) => {
            AppResponse::<Vec<EnvironmentCookie>>::success(success_message("查询成功"), Some(data))
        }
        Err(r) => {
            AppResponse::<Vec<EnvironmentCookie>>::fail(warn_message(&format!("查询失败: {}", r)))
        }
    }
}

pub async fn create(Json(payload): Json<CreatePayload>) -> impl IntoResponse {
    let cookie = EnvironmentCookie {
        value: payload.cookie_str.to_string(),
        environment_uuid: Some(payload.environment_uuid),
        ..Default::default()
    };

    match services::environmnet_cookie::create(&cookie).await {
        Ok(data) => AppResponse::<bool>::success(success_message("创建成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("创建失败: {}", r))),
    }
}

pub async fn modify(Json(payload): Json<ModifyPayload>) -> impl IntoResponse {
    let cookie = EnvironmentCookie {
        value: payload.cookie_str.clone(),
        ..Default::default()
    };

    match services::environmnet_cookie::modify(&payload.environment_uuid, &cookie).await {
        Ok(data) => AppResponse::<bool>::success(success_message("更新成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("更新失败: {}", r))),
    }
}

pub async fn delete(Json(payload): Json<ModifyPayload>) -> impl IntoResponse {
    match services::environmnet_cookie::delete(&payload.environment_uuid).await {
        Ok(data) => AppResponse::<bool>::success(success_message("删除成功"), Some(data)),
        Err(r) => AppResponse::<bool>::fail(warn_message(&format!("删除失败: {}", r))),
    }
}
