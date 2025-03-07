use super::get_user_id;
use crate::response::AppResponse;
use serde_json::Value;
use std::collections::HashMap;

#[tauri::command]
pub async fn browser_start(environment_uuid: &str) -> Result<AppResponse<Value>, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(
        match lp_services::command::start_browser(environment_uuid).await {
            Ok(v) => AppResponse::success(None, Some(v)),
            Err(_) => AppResponse::fail(Some("启动失败".to_string())),
        },
    )
}

pub mod starts {
    use serde_json::json;

    use super::*;

    #[tauri::command]
    pub async fn browser_starts(
        environment_uuids: Vec<String>,
    ) -> Result<AppResponse<HashMap<String, Value>>, tauri::Error> {
        let _ = get_user_id().await?;
        let mut result = HashMap::new();

        for item in environment_uuids {
            match lp_services::command::start_browser(&item.clone()).await {
                Ok(v) => {
                    result.insert(item.clone(), v);
                }
                Err(e) => {
                    result.insert(
                        item.clone(),
                        json!({
                            "environment_id": item.clone(),
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
pub async fn browser_stops(
    environment_uuids: Vec<String>,
) -> Result<AppResponse<HashMap<String, String>>, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(match lp_services::command::stop(environment_uuids).await {
        Ok(v) => AppResponse::success(
            Some("关闭成功，具体关闭信息查看响应数据.".to_string()),
            Some(v),
        ),
        Err(_) => AppResponse::fail(Some("关闭失败".to_string())),
    })
}

#[tauri::command]
pub async fn browser_status() -> Result<AppResponse<HashMap<String, bool>>, tauri::Error> {
    let _ = get_user_id().await?;
    Ok(match lp_services::command::view_active().await {
        Ok(v) => AppResponse::success(Some("查询状态成功.".to_string()), Some(v)),
        Err(_) => AppResponse::fail(Some("查询状态失败".to_string())),
    })
}
