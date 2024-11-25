#[test]
fn test_start_browser() {
    use core::utils::command::start_browser;

    let _ = start_browser("", "", "", "", "").unwrap();
}
