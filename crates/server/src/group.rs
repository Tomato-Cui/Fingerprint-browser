use axum::{extract::Json, response::IntoResponse};
use axum::{routing::post, Router};
use serde::{Deserialize, Serialize};

pub fn build_group_router() -> Router {
    Router::new().nest(
        "/group",
        Router::new()
            .route("/create", post(create_group::handle)) // 浏览器启动
            .route("/update", post(update_group::handle)) // 关闭浏览器
            .route("/list", post(list_group::handle)), // 检查浏览器状态
    )
}

mod create_group {
    use super::*;

    #[derive(Deserialize, Serialize)]
    pub struct Param {}

    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        todo!()
    }
}

mod update_group {
    use super::*;

    #[derive(Deserialize, Serialize)]
    pub struct Param {}

    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        todo!()
    }
}

mod list_group {
    use super::*;

    #[derive(Deserialize, Serialize)]
    pub struct Param {}

    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        todo!()
    }
}
