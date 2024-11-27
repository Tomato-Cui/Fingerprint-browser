#[test]
fn test_new() {
    let config = crate::cores::load_config();
    println!("{:?}", config)
}

#[test]
fn test_get_cache_location() {
    let config = crate::cores::load_config();
    let location = config.get_cache_location();
    println!("{:?}", location)
}

#[test]
fn test_get_user_data_location() {
    let config = crate::cores::load_config();
    let location = config.get_user_data_location();
    println!("{:?}", location)
}

#[test]
fn test_get_user_proxy_location() {
    let config = crate::cores::load_config();
    let location = config.get_user_proxy_location();
    println!("{:?}", location)
}

#[test]
fn test_get_locations() {
    let config = crate::cores::load_config();
    let locations = config.get_locations();
    println!("{:?}", locations);
}
