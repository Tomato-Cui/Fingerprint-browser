#[tokio::test]
async fn test_login() {
    use tokio::time::Instant;

    let now = Instant::now();

    for _ in 1..2000000 {
        tokio::spawn(async {
            for _ in 1..20 {
                cores::request::login("abc", "abc").await.unwrap()
            }
        })
        .await
        .unwrap()
    }

    println!("{:?}", now.elapsed())
}
