#[test]
fn test_app_data_location() {
    use core::utils::common;

    println!("{:?}", common::app_localer::app_data_location());
}

#[test]
fn test_app_location() {
    use core::utils::common;
    println!("{:?}", common::app_localer::app_location());
}

#[test]
fn test_timestamp_to_seconds() {
    use core::utils::common;
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
    use core::utils::common;
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
    use core::utils::common;

    println!("{:?}", common::app_timer::generate_nanosecond_timestamp());
}
