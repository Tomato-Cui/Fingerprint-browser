pub use crate::errors::ApplicationServerError;
use crate::utils::{common::app_localer, encryption};
use std::process::{Child, Command, Stdio};

///启动浏览器
/// TODO:  该功能未完全实现.
pub fn start_browser(
    ua: &str,
    user_data: &str,
    proxy: &str,
    fp_info: &str,
    lang: &str,
) -> Result<Child, ApplicationServerError> {
    let mut args: Vec<String> = vec![
        "--no-default-browser-check".into(),
        "--no-first-run".into(),
        format!("--accept-lang={}", lang),
        format!("--user-agent={}", ua),
        format!(
            "--user-data-dir={}",
            app_localer::app_data_location()
                .join("user_data")
                .join(user_data)
                .to_str()
                .unwrap()
        ),
        format!("--breeze-fp={}", encryption::base64_encode(fp_info)),
    ];
    if proxy != "null" {
        args.push(format!("--proxy-server={}", proxy));
    }

    // let browser_path = app_root().join("BreezeBrowser").join("chrome");
    let browser_path = app_localer::app_location()
        // .join("BreezeBrowser")
        // TODO: WANR: 这里我随便写一个路径都能通过测试.join("test")
        // .join("test")
        .join("C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe");

    println!("{:?}", browser_path);

    let child = Command::new(browser_path)
        .args(args)
        .stderr(Stdio::null()) // 重定向标准错误输出
        .stdout(Stdio::null()) // 重定向标准输出
        .spawn()
        .unwrap();

    Ok(child)
}
