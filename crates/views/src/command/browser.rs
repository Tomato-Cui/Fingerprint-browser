use crate::command::user::get_user_id;
use crate::response::AppResponse;
use std::collections::HashMap;

use serde_json::Value;

#[tauri::command]
pub async fn browser_start(
    group_id: Option<u32>,
    environment_id: u32,
) -> Result<AppResponse<Value>, tauri::Error> {
    let user_id = Some(get_user_id().await);

    Ok(
        match services::command::start_browser(user_id, group_id, environment_id).await {
            Ok(v) => AppResponse::success(None, Some(v)),
            Err(_) => AppResponse::fail(Some("启动失败".to_string())),
        },
    )
}

pub mod starts {
    use serde::Deserialize;
    use serde_json::json;

    use super::*;

    #[derive(Deserialize)]
    pub struct Payload {
        group_id: Option<u32>,
        environment_id: u32,
    }

    #[tauri::command]
    pub async fn browser_starts(
        payload: Vec<Payload>,
    ) -> Result<AppResponse<HashMap<u32, Value>>, tauri::Error> {
        let user_id = get_user_id().await;
        let mut result = HashMap::new();

        for item in payload {
            let (user_id, group_id, environment_id) = (user_id, item.group_id, item.environment_id);

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

        Ok(AppResponse::success(None, Some(result)))
    }
}

#[tauri::command]
pub async fn browser_stops(ids: Vec<i32>) -> Result<AppResponse<HashMap<i32, i32>>, tauri::Error> {
    Ok(match services::command::stop(ids).await {
        Ok(v) => AppResponse::success(
            Some("关闭成功，具体关闭信息查看响应数据.".to_string()),
            Some(v),
        ),
        Err(_) => AppResponse::fail(Some("关闭失败".to_string())),
    })
}

#[tauri::command]
pub async fn browser_status() -> Result<AppResponse<HashMap<i32, bool>>, tauri::Error> {
    Ok(match services::command::view_active().await {
        Ok(v) => AppResponse::success(Some("查询状态成功.".to_string()), Some(v)),
        Err(_) => AppResponse::fail(Some("查询状态失败".to_string())),
    })
}
