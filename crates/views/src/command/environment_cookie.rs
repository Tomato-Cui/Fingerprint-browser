use crate::response::AppResponse;
use models::environmnet_cookie::EnvironmentCookie;

use super::user::get_user_id;

#[tauri::command]
pub async fn environment_cookie_query_environment_uuid(
    environment_uuid: String,
) -> Result<AppResponse<Vec<EnvironmentCookie>>, tauri::Error> {
    let _ = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("查询成功".to_string()), |v| {
        Some(format!("查询失败: {}", v))
    });

    Ok(
        match services::environmnet_cookie::query(&environment_uuid).await {
            Ok(data) => AppResponse::<Vec<EnvironmentCookie>>::success(success_msg, Some(data)),
            Err(r) => AppResponse::<Vec<EnvironmentCookie>>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn environment_cookie_create(
    environment_uuid: String,
    cookie_str: String,
) -> Result<AppResponse<bool>, tauri::Error> {
    let _ = get_user_id().await?;
    let (success_msg, warn_msg) = (Some("创建成功".to_string()), |v| {
        Some(format!("创建失败: {}", v))
    });

    let cookie = EnvironmentCookie {
        value: cookie_str.to_string(),
        environment_uuid: Some(environment_uuid),
        ..Default::default()
    };

    Ok(match services::environmnet_cookie::create(&cookie).await {
        Ok(data) => {
            if data {
                AppResponse::<bool>::success(success_msg, Some(data))
            } else {
                AppResponse::<bool>::fail(warn_msg("未知错误".to_string()))
            }
        }
        Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
    })
}

#[tauri::command]
pub async fn environment_cookie_modify(
    environment_uuid: String,
    cookie_str: String,
) -> Result<AppResponse<bool>, tauri::Error> {
    let _ = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("更新成功".to_string()), |v| {
        Some(format!("更新失败: {}", v))
    });

    let cookie = EnvironmentCookie {
        value: cookie_str.clone(),
        ..Default::default()
    };

    Ok(
        match services::environmnet_cookie::modify(&environment_uuid, &cookie).await {
            Ok(data) => {
                if data {
                    AppResponse::<bool>::success(success_msg, Some(data))
                } else {
                    AppResponse::<bool>::fail(warn_msg("未知错误".to_string()))
                }
            }
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        },
    )
}

#[tauri::command]
pub async fn environment_cookie_delete(
    environmnet_uuid: &str,
) -> Result<AppResponse<bool>, tauri::Error> {
    let _ = get_user_id().await?;

    let (success_msg, warn_msg) = (Some("删除成功".to_string()), |v| {
        Some(format!("删除失败: {}", v))
    });

    Ok(
        match services::environmnet_cookie::delete(&environmnet_uuid).await {
            Ok(data) => {
                if data {
                    AppResponse::<bool>::success(success_msg, Some(data))
                } else {
                    AppResponse::<bool>::fail(warn_msg("未知错误".to_string()))
                }
            }
            Err(r) => AppResponse::<bool>::fail(warn_msg(r.to_string())),
        },
    )
}
