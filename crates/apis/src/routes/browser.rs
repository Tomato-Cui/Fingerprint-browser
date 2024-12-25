use crate::response::AppResponse;
use axum::extract::Json;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use serde_json::Value;

pub fn build_router() -> Router {
    Router::new().nest(
        "/browsers",
        Router::new()
            .route("/start/:uuid", get(start::handle))
            .route("/starts", post(starts::handle))
            .route("/stops", post(stops::handle))
            .route("/status", get(status::handle)),
    )
}

mod start {
    use super::*;
    use axum::extract::Path;

    pub async fn handle(Path(uuid): Path<String>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("开启成功".to_string()), |v| {
            Some(format!("开启失败: {}", v))
        });

        match services::command::start_browser(&uuid).await {
            Ok(data) => AppResponse::<Value>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        }
    }
}

mod starts {
    use std::collections::HashMap;

    use super::*;
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize)]
    pub struct Payload {
        pub environment_uuids: Vec<String>,
    }

    pub async fn handle(Json(payload): Json<Payload>) -> impl IntoResponse {
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
}

mod stops {
    use serde::Deserialize;

    use super::*;
    use std::collections::HashMap;

    #[derive(Deserialize)]
    pub struct Payload {
        pub environment_uuids: Vec<String>,
    }

    pub async fn handle(Json(payload): Json<Payload>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("关闭成功".to_string()), |v| {
            Some(format!("关闭失败: {}", v))
        });

        match services::command::stop(payload.environment_uuids).await {
            Ok(data) => AppResponse::<HashMap<String, i32>>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<HashMap<String, i32>>::fail(warn_msg(r.to_string())),
        }
    }
}

mod status {
    use std::collections::HashMap;

    use super::*;

    pub async fn handle() -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
            Some(format!("查询失败: {}", v))
        });

        match services::command::view_active().await {
            Ok(data) => AppResponse::<HashMap<String, bool>>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<HashMap<String, bool>>::fail(warn_msg(r.to_string())),
        }
    }
}
