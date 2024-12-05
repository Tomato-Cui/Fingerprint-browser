pub mod auth {
    use cores::auth::{clear_token, init_auth_state, set_token};
    use cores::utils::response::AppResponse;

    #[tauri::command]
    pub async fn login(token: &str) -> Result<AppResponse<bool>, tauri::Error> {
        init_auth_state().await;
        set_token(token).await;
        Ok(AppResponse::success(None, Some(true)))
    }

    #[tauri::command]
    pub async fn logout() -> Result<AppResponse<bool>, tauri::Error> {
        clear_token().await;
        Ok(AppResponse::success(None, Some(true)))
    }
}

pub mod environment {
    use cores::utils::response::AppResponse;

    #[tauri::command]
    pub async fn create() -> Result<AppResponse<bool>, tauri::Error> {
        todo!()
    }

    #[tauri::command]
    pub async fn delete() -> Result<AppResponse<bool>, tauri::Error> {
        todo!()
    }

    #[tauri::command]
    pub async fn update() -> Result<AppResponse<bool>, tauri::Error> {
        todo!()
    }

    #[tauri::command]
    pub async fn move_group() -> Result<AppResponse<bool>, tauri::Error> {
        todo!()
    }
}

pub mod browser {
    use std::collections::HashMap;

    use cores::{
        apis::browser::{self, StartEnvironmentParams},
        auth::get_token,
        utils::response::AppResponse,
    };
    use serde_json::Value;

    #[tauri::command]
    pub async fn starts(
        environments: Vec<StartEnvironmentParams>,
    ) -> Result<AppResponse<HashMap<i32, Value>>, tauri::Error> {
        let token = get_token().await;
        println!("{:?}", token);
        Ok(match browser::starts(environments).await {
            Ok(v) => v,
            Err(_) => AppResponse::fail(Some("启动失败".to_string())),
        })
    }

    #[tauri::command]
    pub async fn stops(env_ids: Vec<i32>) -> Result<AppResponse<HashMap<i32, i32>>, tauri::Error> {
        Ok(match browser::stop(env_ids).await {
            Ok(v) => v,
            Err(_) => AppResponse::fail(Some("关闭失败".to_string())),
        })
    }

    #[tauri::command]
    pub async fn status() -> Result<AppResponse<HashMap<i32, bool>>, tauri::Error> {
        Ok(match browser::view_active().await {
            Ok(v) => v,
            Err(_) => AppResponse::fail(Some("查询失败".to_string())),
        })
    }
}
