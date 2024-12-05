pub mod auth {
    use cores::auth::set_token;
    use cores::utils::response::AppResponse;

    #[tauri::command]
    pub async fn login(token: &str) -> Result<AppResponse<bool>, tauri::Error> {
        set_token(token).await;
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
        utils::response::AppResponse,
    };

    #[tauri::command]
    pub async fn starts(
        browsers: Vec<StartEnvironmentParams>,
    ) -> Result<AppResponse<HashMap<i32, bool>>, tauri::Error> {
        Ok(match browser::starts(browsers).await {
            Ok(v) => v,
            Err(_) => AppResponse::fail(Some("启动失败".to_string())),
        })
    }

    #[tauri::command]
    pub async fn stops(
        browser_ids: Vec<i32>,
    ) -> Result<AppResponse<HashMap<i32, i32>>, tauri::Error> {
        Ok(match browser::stop(browser_ids).await {
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
