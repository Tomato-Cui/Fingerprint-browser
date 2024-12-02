use cores::auth;

pub mod test_backend;

#[allow(dead_code)]
pub async fn setup() {
    auth::init_auth_state().await;
}

#[tokio::test]
pub async fn test_get_json_chrome_resoures() {
    let url = "https://googlechromelabs.github.io/chrome-for-testing/latest-versions-per-milestone-with-downloads.json";
    let result = cores::requests::browser_resources::chrome::Action::new(url)
        .await
        .unwrap();

    println!("{:?}", result.get_all_version());
    println!("{:?}", result.get_platform("113"));
}

#[tokio::test]
pub async fn download_file() {
    let url = "https://storage.googleapis.com/chrome-for-testing-public/113.0.5672.63/linux64/chrome-linux64.zip";
    cores::requests::download_file(url, "./hello.zip")
        .await
        .unwrap();
}
