#[tokio::test]
async fn test_new() {
    use cores::config::get_config;
    crate::cores::init_config().await;
    println!("{:?}", get_config())
}

#[tokio::test]
async fn test_get_cache_location() {
    crate::cores::init_config().await;
    use cores::config::get_config;
    let location = get_config().unwrap().get_cache_location().await;
    println!("{:?}", location)
}

#[tokio::test]
async fn test_get_user_data_location() {
    crate::cores::init_config().await;
    use cores::config::get_config;
    let location = get_config().unwrap().get_user_data_location().await;
    println!("{:?}", location)
}

#[tokio::test]
async fn test_get_user_proxy_location() {
    use cores::config::get_config;

    crate::cores::init_config().await;
    let location = get_config().unwrap().get_user_proxy_location().await;
    println!("{:?}", location)
}

#[tokio::test]
async fn test_get_locations() {
    use cores::config::get_config;

    crate::cores::init_config().await;
    let locations = get_config().unwrap().get_locations().await;
    println!("{:?}", locations);
}

#[tokio::test]
async fn test_get_resource_url() {
    use cores::config::get_config;

    crate::cores::init_config().await;
    let locations = &get_config().unwrap().browser.chrome.resource_url;
    println!("{:?}", locations);
}
