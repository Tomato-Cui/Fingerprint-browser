#[tokio::test]
async fn test_login() {
    use tokio::time::Instant;
    let now = Instant::now();

    tokio::spawn(async { cores::request::login("abc", "abc").await.unwrap() })
        .await
        .unwrap();

    println!("{:?}", now.elapsed())
}
