use axum::{extract::Json, response::IntoResponse};
use axum::{routing::post, Router};
use serde::{Deserialize, Serialize};

pub fn build_router() -> Router {
    Router::new().nest("/users", Router::new().route("/login", post(login::handle)))
}

mod login {
    use cores::{requests::backend::auth, utils::response::AppResponse};

    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Param {
        username: String,
        password: String,
    }

    pub async fn handle(Json(payload): Json<Param>) -> impl IntoResponse {
        match auth::login(&payload.username, &payload.password).await {
            Ok(res) => AppResponse::<()>::success(res.message, None),
            Err(r) => AppResponse::<()>::success(Some(r.to_string()), None),
        }
    }
}
