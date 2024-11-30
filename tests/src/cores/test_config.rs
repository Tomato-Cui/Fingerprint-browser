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
    let location = get_config().unwrap().get_cache_location();
    println!("{:?}", location)
}

#[tokio::test]
async fn test_get_user_data_location() {
    crate::cores::init_config().await;
    use cores::config::get_config;
    let location = get_config().unwrap().get_user_data_location();
    println!("{:?}", location)
}

#[tokio::test]
async fn test_get_user_proxy_location() {
    crate::cores::init_config().await;
    use cores::config::get_config;
    let location = get_config().unwrap().get_user_proxy_location();
    println!("{:?}", location)
}

#[tokio::test]
async fn test_get_locations() {
    use cores::config::get_config;
    let locations = get_config().unwrap().get_locations();
    println!("{:?}", locations);
}
