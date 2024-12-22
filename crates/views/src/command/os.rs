use crate::response::AppResponse;

#[tauri::command]
pub async fn platform() -> Result<AppResponse<String>, tauri::Error> {
    let os = std::env::consts::OS.to_string();
    Ok(AppResponse::success(None, Some(os)))
}
