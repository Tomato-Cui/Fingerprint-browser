use lp_cores::requests::JsonRespnse;
use serde_json::{json, Value};

#[tauri::command]
pub async fn latest_kernel() -> Result<JsonRespnse, tauri::Error> {
    Ok(lp_services_remote::requests::kernel_versions::latest_kernel().await?)
}

/// 接收version，判断该version是否存在。
#[tauri::command]
pub async fn kernel_location(
    version: &str,
) -> Result<crate::response::AppResponse<Value>, tauri::Error> {
    let (success_msg, _warn_msg) = (Some("获取成功".to_string()), |v: &str| {
        Some(format!("获取失败: {}", v))
    });

    let mut cache_location = lp_states::config::APP_DATA.clone();
    let kernel_location = lp_states::config::get_config()
        .map(|v| v.app.location.browser_drivers_location.clone())
        .unwrap_or_else(|| "fatast-kernels".to_string());

    cache_location = cache_location.join(kernel_location).join(version);

    Ok(crate::response::AppResponse::<Value>::success(
        success_msg,
        Some(json!({
            "exist": cache_location.exists(),
            "location": cache_location.to_str().unwrap()
        })),
    ))
}

#[tauri::command]
pub async fn unzip_kernel_zip(
    location: &str,
    extract_to: &str,
) -> Result<crate::response::AppResponse<bool>, tauri::Error> {
    let (success_msg, _warn_msg) = (Some("安装成功".to_string()), |v: &str| {
        Some(format!("安装失败: {}", v))
    });

    lp_commons::util::unzip(location, extract_to).expect("kernel unzip failed.");

    std::fs::remove_file(location)?;

    Ok(crate::response::AppResponse::<bool>::success(
        success_msg,
        Some(true),
    ))
}
