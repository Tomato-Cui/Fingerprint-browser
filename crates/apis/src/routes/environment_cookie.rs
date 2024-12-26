use crate::response::AppResponse;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use models::environmnet_cookie::EnvironmentCookie;

pub fn build_router() -> Router {
    Router::new().nest(
        "/environmnet-cookies",
        Router::new()
            .route("/query/:uuid", get(query_by_uuid::handle))
            .route("/create/:uuid", post(create::handle))
            .route("/modify/:uuid", put(modify::handle))
            .route("/delete/:id", delete(delete::handle)),
    )
}

mod query_by_uuid {

    use super::*;

    pub async fn handle(Path(uuid): Path<String>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::environmnet_cookie::query(&uuid).await {
            Ok(data) => AppResponse::<Vec<EnvironmentCookie>>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Vec<EnvironmentCookie>>::fail(warn_msg(r.to_string())),
        }
    }
}

mod create {
    use super::*;

    #[derive(serde::Deserialize)]
    pub struct Payload {
        pub cookie_str: String,
    }

    pub async fn handle(Path(uuid): Path<String>, payload: Json<Payload>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
            Some(format!("创建失败: {}", v))
        });

        let cookie = EnvironmentCookie {
            value: payload.cookie_str.to_string(),
            environment_uuid: Some(uuid),
            ..Default::default()
        };

        match services::environmnet_cookie::create(&cookie).await {
            Ok(data) => {
                if data {
                    AppResponse::<()>::success(success_msg, Some(()))
                } else {
                    AppResponse::<()>::fail(warn_msg("未知错误".to_string()))
                }
            }
            Err(r) => AppResponse::<()>::fail(warn_msg(r.to_string())),
        }
    }
}

mod modify {
    use super::*;

    #[derive(serde::Deserialize)]
    pub struct Payload {
        pub cookie_str: String,
    }

    pub async fn handle(
        Path(environment_uuid): Path<String>,
        payload: Json<Payload>,
    ) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
            Some(format!("更新失败: {}", v))
        });
        let cookie = EnvironmentCookie {
            value: payload.cookie_str.clone(),
            ..Default::default()
        };

        match services::environmnet_cookie::modify(&environment_uuid, &cookie).await {
            Ok(data) => {
                if data {
                    AppResponse::<()>::success(success_msg, Some(()))
                } else {
                    AppResponse::<()>::fail(warn_msg("未知错误".to_string()))
                }
            }
            Err(r) => AppResponse::<()>::fail(warn_msg(r.to_string())),
        }
    }
}

mod delete {
    use super::*;

    pub async fn handle(Path(environment_uuid): Path<String>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
            Some(format!("删除失败: {}", v))
        });

        match services::environmnet_cookie::delete(&environment_uuid).await {
            Ok(data) => {
                if data {
                    AppResponse::<()>::success(success_msg, Some(()))
                } else {
                    AppResponse::<()>::fail(warn_msg("未知错误".to_string()))
                }
            }
            Err(r) => AppResponse::<()>::fail(warn_msg(r.to_string())),
        }
    }
}
