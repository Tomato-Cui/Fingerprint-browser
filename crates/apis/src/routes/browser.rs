use crate::middlewares;
use crate::response::AppResponse;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Extension;
use axum::{Json, Router};
use serde_json::Value;

pub fn build_router() -> Router {
    Router::new().nest(
        "/browsers",
        Router::new()
            .route("/start/:id", get(start::handle))
            .route("/starts", post(starts::handle))
            .route("/stops", post(stops::handle))
            .route("/status", get(status::handle)),
    )
}

mod start {
    use super::*;
    use axum::extract::Path;
    use middlewares::CurrentUser;

    pub async fn handle(state: Extension<CurrentUser>, Path(id): Path<u32>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("开启成功".to_string()), |v| {
            Some(format!("开启失败: {}", v))
        });

        let (id, user_id) = (id, state.id);
        match services::command::start_browser(Some(user_id), None, id).await {
            Ok(data) => AppResponse::<Value>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        }
    }
}

mod starts {
    use std::collections::HashMap;

    use super::*;
    use middlewares::CurrentUser;
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize)]
    pub struct Payload {
        pub group_id: Option<u32>,
        pub id: u32,
    }

    pub async fn handle(
        state: Extension<CurrentUser>,
        Json(payload): Json<Vec<Payload>>,
    ) -> impl IntoResponse {
        let user_id = state.id;

        let mut result = HashMap::new();
        for item in payload {
            let (user_id, group_id, environment_id) = (user_id, item.group_id, item.id);

            match services::command::start_browser(Some(user_id), group_id, environment_id).await {
                Ok(v) => {
                    result.insert(environment_id, v);
                }
                Err(e) => {
                    result.insert(
                        environment_id,
                        json!({
                            "environment_id": environment_id,
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
        pub ids: Vec<i32>,
    }

    pub async fn handle(Json(payload): Json<Payload>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("关闭成功".to_string()), |v| {
            Some(format!("关闭失败: {}", v))
        });

        match services::command::stop(payload.ids).await {
            Ok(data) => AppResponse::<HashMap<i32, i32>>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<HashMap<i32, i32>>::fail(warn_msg(r.to_string())),
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
            Ok(data) => AppResponse::<HashMap<i32, bool>>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<HashMap<i32, bool>>::fail(warn_msg(r.to_string())),
        }
    }
}
