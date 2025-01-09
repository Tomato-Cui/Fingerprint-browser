use crate::response::AppResponse;
use services_remote::requests::browser;

use cores::request::JsonRespnse;

#[tauri::command]
pub async fn browser_start(
    environment_uuid: &str,
) -> Result<AppResponse<JsonRespnse>, tauri::Error> {
    Ok(match browser::start(environment_uuid).await {
        Ok(v) => AppResponse::success(None, Some(v)),
        Err(_) => AppResponse::fail(Some("启动失败".to_string())),
    })
}

pub mod starts {
    use super::*;

    #[tauri::command]
    pub async fn browser_starts(
        environment_uuids: Vec<String>,
    ) -> Result<AppResponse<JsonRespnse>, tauri::Error> {
        Ok(match browser::starts(environment_uuids).await {
            Ok(v) => AppResponse::success(None, Some(v)),
            Err(_) => AppResponse::fail(Some("启动失败".to_string())),
        })
    }
}

#[tauri::command]
pub async fn browser_stops(
    environment_uuids: Vec<String>,
) -> Result<AppResponse<JsonRespnse>, tauri::Error> {
    Ok(match browser::stops(environment_uuids).await {
        Ok(v) => AppResponse::success(
            Some("关闭成功，具体关闭信息查看响应数据.".to_string()),
            Some(v),
        ),
        Err(_) => AppResponse::fail(Some("关闭失败".to_string())),
    })
}

#[tauri::command]
pub async fn browser_status() -> Result<AppResponse<JsonRespnse>, tauri::Error> {
    Ok(match browser::status().await {
        Ok(v) => AppResponse::success(Some("查询状态成功.".to_string()), Some(v)),
        Err(_) => AppResponse::fail(Some("查询状态失败".to_string())),
    })
}
