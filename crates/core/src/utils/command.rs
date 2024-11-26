use std::process::{ExitStatus, Stdio};

use serde_json::to_string;
use tokio::process::{Child, Command};

pub use crate::errors::ApplicationServerError;
use crate::{
    apis::{browser::update_browser_status_handle, Result},
    config,
    models::ua::Ua,
    utils::{common::app_localer, encryption},
};
// use std::process::{Child, Command, ExitStatus, Stdio};

use super::response::AppResponse;

/// 启动浏览器
/// TODO:  该功能未完全实现.
pub async fn start_browser(
    ua: Ua,
    user_data: &str,
    proxy: &str,
    fp_info: &str,
    lang: &str,
    browser_exe_path: &str,
    browser_id: i8,
    browser_unique_key: &str,
) -> core::result::Result<Child, ApplicationServerError> {
    let breeze_fp = format!("--breeze-fp={}", encryption::base64_encode(fp_info));
    let user_agent = format!("--user-agent={}", to_string(&ua)?);
    let accept_lang = format!("--accept-lang={}", lang);
    let no_first_run = "--no-first-run".to_string();
    let user_data_dir = format!(
        "--user-data-dir={}",
        config::AConfig
            .get_user_data_location()
            .join(user_data)
            .to_str()
            .unwrap()
    );
    let no_default_browser_check = "--no-default-browser-check".to_string();
    let browser_unique = format!(
        "--app-browser-unique = {}.{}",
        config::AConfig.app.id,
        browser_unique_key
    );

    let mut args: Vec<String> = vec![
        no_default_browser_check,
        no_first_run,
        accept_lang,
        user_agent,
        user_data_dir,
        breeze_fp,
        browser_unique,
    ];

    args.push(if proxy.is_empty() {
        format!("--proxy-server={}", proxy)
    } else {
        "".to_string()
    });

    let browser_path = app_localer::app_location()
        // TODO: WANR: 这里我随便写一个路径都能通过测试.join("test")
        // .join("BreezeBrowser")
        // .join("test")
        .join(browser_exe_path);

    let child = Command::new(browser_path)
        .args(args)
        .stderr(Stdio::null()) // 重定向标准错误输出
        .stdout(Stdio::null()) // 重定向标准输出
        .spawn()
        .unwrap();

    // 更新浏览器的启动状体
    update_browser_status_handle(browser_id, true)?;

    Ok(child)
}

/// 关闭浏览器， 需要提供浏览器的ID
pub async fn stop_browser(child: &mut Child, browser_id: i8) -> Result<AppResponse<ExitStatus>> {
    let _ = child.kill().await?;
    let exit_status = child.wait().await?;

    update_browser_status_handle(browser_id, false)?;
    Ok(AppResponse::success(
        Some("close browser finally !".to_string()),
        Some(exit_status),
    ))
}
