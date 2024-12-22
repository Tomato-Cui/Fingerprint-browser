use crate::response::AppResponse;
use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn build_router() -> Router {
    Router::new().nest(
        "/users",
        Router::new()
            .route("/login", post(login::handle))
            .route("/logout", get(logout::handle))
            .route("/register", post(register::handle))
            .route("/reset-password", post(reset_password::handle)),
        // .route("/register/send", get(register_send::handle))
        // .route("/reset_password/send", get(reset_password_send::handle)),
    )
}

mod login {

    use serde_json::json;

    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Payload {
        pub username: String,
        pub password: String,
    }

    pub async fn handle(Json(payload): Json<Payload>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("登录成功".to_string()), |v| {
            Some(format!("登录失败: {}", v))
        });

        match services::user::login(&payload.username, &payload.password).await {
            Ok(token) => AppResponse::<Value>::success(
                success_msg,
                Some(json!({
                    "token": token,
                })),
            ),
            Err(r) => AppResponse::<Value>::fail(warn_msg(r.to_string())),
        }
    }
}

mod logout {
    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    struct Payload {
        pub username: String,
        pub password: String,
    }
    pub async fn handle() -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("退出成功".to_string()), |v| {
            Some(format!("退出失败: {}", v))
        });

        match services::user::logout().await {
            Ok(ok) => {
                if ok {
                    AppResponse::<()>::success(success_msg, Some(()))
                } else {
                    AppResponse::<()>::fail(warn_msg("未知错误".to_string()))
                }
            }
            Err(r) => AppResponse::<()>::fail(warn_msg(r.to_string())),
        }
    }
}

mod register {
    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Payload {
        pub email: String,
        pub username: String,
        pub password: String,
    }
    pub async fn handle(payload: Json<Payload>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("注册成功".to_string()), |v| {
            Some(format!("注册失败: {}", v))
        });

        match services::user::regsiter(&payload.email, &payload.username, &payload.password).await {
            Ok(ok) => {
                if ok {
                    AppResponse::<()>::success(success_msg, Some(()))
                } else {
                    AppResponse::<()>::fail(warn_msg("未知错误".to_string()))
                }
            }
            Err(r) => AppResponse::<()>::fail(warn_msg(r.to_string())),
        }
    }
}
mod reset_password {
    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Payload {
        pub email: String,
        pub password1: String,
        pub password2: String,
    }

    pub async fn handle(payload: Json<Payload>) -> impl IntoResponse {
        let (success_msg, warn_msg) = (Some("重置密码成功".to_string()), |v| {
            Some(format!("重置密码失败: {}", v))
        });

        match services::user::reset_password(&payload.email, &payload.password1, &payload.password2)
            .await
        {
            Ok(ok) => {
                if ok {
                    AppResponse::<()>::success(success_msg, Some(()))
                } else {
                    AppResponse::<()>::fail(warn_msg("未知错误".to_string()))
                }
            }
            Err(r) => AppResponse::<()>::fail(warn_msg(r.to_string())),
        }
    }
}

#[allow(dead_code)]
mod register_send {
    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    struct Payload {
        pub code: String,
        pub email: String,
        pub username: String,
        pub password: String,
    }
    pub async fn handle() -> impl IntoResponse {}
}

#[allow(dead_code)]
mod reset_password_send {
    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    struct Payload {
        pub email: String,
    }
    pub async fn handle() -> impl IntoResponse {}
}
