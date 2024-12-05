#[tokio::test]
async fn test_start_stop_browser() {
    use cores::models::enviroment::{self};
    use cores::models::fingerprint::{self};
    use cores::utils::command::BrowserChildInfo;
    use cores::utils::command::Processer;
    use cores::utils::common::get_debug_port;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::Mutex;

    crate::cores::init_config().await;

    let processer = Arc::new(Mutex::new(Processer::new()));

    let fingerprint = fingerprint::Fingerprint::query_fingerprint_by_id(1)
        .await
        .unwrap();
    let environment = enviroment::Environment::query_envirionment_by_id(1)
        .await
        .unwrap();

    let port = get_debug_port().await.unwrap();
    // let proxy = get_proxy_from_registry().unwrap();

    let payload = BrowserChildInfo::new(
        environment,
        fingerprint,
        port,
        "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
    );
    processer.lock().await.start_browser(payload).await.unwrap();

    tokio::time::sleep(Duration::from_secs(5)).await;

    let _ = processer.lock().await.status(1).await.unwrap();
}

#[tokio::test]
async fn test_thirtyfour() {
    // TODO: 该接口未写，因为Command:new 打开浏览器目前可以正常使用，那么就先不改了。
    use thirtyfour::prelude::*;
    let caps = DesiredCapabilities::chrome();
    let _driver = WebDriver::new("http://localhost:9515", caps).await.unwrap();
}
