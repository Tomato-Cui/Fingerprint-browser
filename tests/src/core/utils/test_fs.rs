#[test]
fn test_delete_data_file() {
    use core::utils::fs::delete_data_file;

    delete_data_file("datas", "user_proxy").unwrap();
}
