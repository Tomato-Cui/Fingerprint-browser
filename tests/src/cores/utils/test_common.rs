#[test]
fn test_app_data_location() {
    use cores::utils::common;

    println!("{:?}", common::app_localer::app_data_location());
}

#[test]
fn test_app_location() {
    use cores::utils::common;
    println!("{:?}", common::app_localer::app_location());
}

#[test]
fn test_timestamp_to_seconds() {
    use cores::utils::common;
    use std::time::{SystemTime, UNIX_EPOCH};

    println!(
        "{:?}",
        common::app_timer::timestamp_to_seconds(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        )
    );
}

#[test]
fn test_seconds_to_timestamp() {
    use cores::utils::common;
    use std::time::{SystemTime, UNIX_EPOCH};

    println!(
        "{:?}",
        common::app_timer::seconds_to_timestamp(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64()
        )
    );
}

#[test]
fn test_generate_nanosecond_timestamp() {
    use cores::utils::common;

    println!("{:?}", common::app_timer::generate_nanosecond_timestamp());
}

#[test]
fn test_get_proxy_from_registry() {
    use cores::utils::common;

    println!("{:?}", common::get_proxy_from_registry());
}

#[test]
fn test_get_chrome_install_path() {
    use cores::utils::common;
    println!("{:?}", common::get_chrome_install_path());
}
