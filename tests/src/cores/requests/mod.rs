pub mod test_backend;

#[allow(dead_code)]
pub async fn setup() {
    use cores::auth;
    auth::init_auth_state().await;
}

#[tokio::test]
pub async fn test_get_json_chrome_resoures() {
    let url = "https://googlechromelabs.github.io/chrome-for-testing/known-good-versions-with-downloads.json";
    let result = cores::requests::browser_resources::chrome::Action::new(url)
        .await
        .unwrap();

    println!("{:?}", result.get_all_version());
    println!("{:?}", result.get_platform("113"));
}

#[tokio::test]
pub async fn download_file() {
    let url = "https://googlechromelabs.github.io/chrome-for-testing/known-good-versions-with-downloads.json";
    cores::requests::download_file(url, "./hello.zip")
        .await
        .unwrap();
}

#[tokio::test]
pub async fn test_get_current_platform() {
    use cores::apis::browser_driver::get_current_platform;

    crate::cores::init_config().await;
    let t = get_current_platform(cores::apis::browser_driver::BrowserTye::Chrome, "131.0.6761.0").await;
    println!("{:?}", t)
}

#[tokio::test]
pub async fn test_get_all_version() {
    use cores::apis::browser_driver::get_all_version;

    crate::cores::init_config().await;
    let t = get_all_version(cores::apis::browser_driver::BrowserTye::Chrome).await;
    println!("{:?}", t)
}
