#[tokio::test]
async fn test_login() {
    use tokio::time::Instant;
    let now = Instant::now();

    tokio::spawn(async { cores::request::auth::login("lius", "lius").await.unwrap() })
        .await
        .unwrap();

    println!("{:?}", now.elapsed())
}

#[tokio::test]
async fn test_register() {
    use tokio::time::Instant;
    let now = Instant::now();

    tokio::spawn(async {
        cores::request::auth::register("lius", "lius", "liushuinew@163.com", "681397")
            .await
            .unwrap()
    })
    .await
    .unwrap();

    println!("{:?}", now.elapsed())
}

#[tokio::test]
async fn test_send() {
    use tokio::time::Instant;
    let now = Instant::now();

    tokio::spawn(async {
        cores::request::auth::send("liushuinew@163.com")
            .await
            .unwrap()
    })
    .await
    .unwrap();

    println!("{:?}", now.elapsed())
}
