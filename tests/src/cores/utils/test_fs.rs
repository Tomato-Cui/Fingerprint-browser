#[tokio::test]
async fn test_delete_data_file() {
    use cores::utils::fs::delete_data_file;

    delete_data_file("datas", "user_proxy").await.unwrap();
}
