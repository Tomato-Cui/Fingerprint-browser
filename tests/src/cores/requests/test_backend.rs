#[tokio::test]
async fn test_login() {
    use cores::auth::get_token;
    use tokio::time::Instant;
    let now = Instant::now();

    cores::requests::backend::auth::login("lius", "lius")
        .await
        .unwrap();
    println!("{}", get_token().await.unwrap());
    println!("{:?}", now.elapsed())
}

#[tokio::test]
async fn test_register() {
    use tokio::time::Instant;
    let now = Instant::now();

    let msg =
        cores::requests::backend::auth::register("lius", "lius", "liushuinew@163.com", "681397")
            .await
            .unwrap();

    println!("{}", msg.unwrap());
    println!("{:?}", now.elapsed())
}

#[tokio::test]
async fn test_send() {
    use tokio::time::Instant;
    let now = Instant::now();

    let msg = cores::requests::backend::auth::send("lius@163.com")
        .await
        .unwrap();

    println!("{}", msg.unwrap());
    println!("{:?}", now.elapsed())
}
