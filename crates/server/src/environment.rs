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

    #[derive(Deserialize, Serialize)]
    pub struct Param {}

    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        todo!()
    }
}

mod update_environment {
    use super::*;

    #[derive(Deserialize, Serialize)]
    pub struct Param {}

    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        todo!()
    }
}

mod list_environment {
    use super::*;

    #[derive(Deserialize, Serialize)]
    pub struct Param {}

    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        todo!()
    }
}

mod delete_environment {
    use super::*;

    #[derive(Deserialize, Serialize)]
    pub struct Param {}

    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        todo!()
    }
}

mod regroup_environment {
    use super::*;

    #[derive(Deserialize, Serialize)]
    pub struct Param {}

    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        todo!()
    }
}

mod delete_cache_environment {
    use super::*;

    #[derive(Deserialize, Serialize)]
    pub struct Param {}

    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        todo!()
    }
}
