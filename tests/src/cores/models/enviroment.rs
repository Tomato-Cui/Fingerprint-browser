#[tokio::test]
async fn test_environment_insert() {
    use cores::models::enviroment::{self, Environment};

    crate::cores::models::load_db().await;

    let environ = Environment {
          id: Some(2),
        name: String::from("development Environment"),
        description:None,
        owner_id: String::from("abc123"),
        domain_name: String::from("dev.example.com"),
        open_urls: Some(String::from("http://dev.example.com, http://test.example.com")),
        repeat_config: Some(String::from("{\"rule\":\"unique\"}")),
        username: String::from("devuser"),
        password: String::from("devpassword123"),
        fakey: String::from("devfakey123456"),
        cookie: Some(String::from("sessionid=xyzdev123; path=/; domain=dev.example.com")),
        ignore_cookie_error: Some(0),
        group_id: Some(1),
        fp_info_id: Some(1),
        ua: String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"),
        os: String::from("Windows 10"),
        country: Some(String::from("US")),
        region: Some(String::from("California")),
        city: Some(String::from("Los Angeles")),
        remark: Some(String::from("Development environment for testing.")),
        ipchecker: String::from("https://ip-api.com/json"),
        sys_app_cate_id: String::from("devcat_001"),
        user_proxy_config: Some(String::from("{\"proxy_type\":\"http\",\"proxy_host\":\"proxy.dev.example.com\",\"proxy_port\":8080}")),
        proxy: Some(String::from("192.168.1.101:8080")),
        proxy_enable: 1,
        is_tz: 1,
        is_pos: 1,
        user_data_file: String::from("/path/to/dev/user/data/file"),
        driver_location: Some(String::from("/path/to/dev/driver")),
        status: 0,
        created_at: Some(String::from("2024-02-01T10:00:00Z")),
        updated_at: Some(String::from("2024-02-01T10:30:00Z")),
        lasted_at: Some(String::from("2024-02-01T10:30:00Z")),
        deleted_at: None,
    };
    enviroment::Environment::insert(environ).await.unwrap();
}

#[tokio::test]
async fn test_environment_query() {
    use cores::apis::PageParam;
    use cores::models::enviroment::{self};

    crate::cores::models::load_db().await;
    let result = enviroment::Environment::query_envirionment(&PageParam {
        page_num: Some(1),
        page_size: Some(10),
    })
    .await;

    println!("{:?}", result);
}

#[tokio::test]
async fn test_environment_query_by_id() {
    use cores::models::enviroment::{self};

    crate::cores::models::load_db().await;
    let result = enviroment::Environment::query_envirionment_by_id(1000).await;

    println!("{:?}", result);
}

#[tokio::test]
async fn test_environment_update() {
    use cores::models::enviroment::Environment;
    use cores::models::enviroment::{self};
    crate::cores::models::load_db().await;

    let environ = Environment {
        id: Some(2),
        name: String::from("PRODUCTION Environment"),
        description: None,
        owner_id: String::from("abc123"),
        domain_name: String::from("dev.example.com"),
        open_urls: Some(String::from("http://dev.example.com, http://test.example.com")),
        repeat_config: Some(String::from("{\"rule\":\"unique\"}")),
        username: String::from("devuser"),
        password: String::from("devpassword123"),
        fakey: String::from("devfakey123456"),
        cookie: Some(String::from("sessionid=xyzdev123; path=/; domain=dev.example.com")),
        ignore_cookie_error: Some(0),
        group_id: Some(1),
        fp_info_id: Some(1),
        ua: String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"),
        os: String::from("Windows 10"),
        country: Some(String::from("US")),
        region: Some(String::from("California")),
        city: Some(String::from("Los Angeles")),
        remark: Some(String::from("Development environment for testing.")),
        ipchecker: String::from("https://ip-api.com/json"),
        sys_app_cate_id: String::from("devcat_001"),
        user_proxy_config: Some(String::from("{\"proxy_type\":\"http\",\"proxy_host\":\"proxy.dev.example.com\",\"proxy_port\":8080}")),
        proxy: Some(String::from("192.168.1.101:8080")),
        proxy_enable: 1,
        is_tz: 1,
        is_pos: 1,
        user_data_file: String::from("/path/to/dev/user/data/file"),
        driver_location: Some(String::from("/path/to/dev/driver")),
        status: 0,
        created_at: Some(String::from("2024-02-01T10:00:00Z")),
        updated_at: Some(String::from("2024-02-01T10:30:00Z")),
        lasted_at: Some(String::from("2024-02-01T10:30:00Z")),
        deleted_at: None,
    };

    let result = enviroment::Environment::update_environment(environ).await;

    println!("{:?}", result);
}

#[tokio::test]
async fn test_environment_update_status() {
    use cores::models::enviroment::{self};

    crate::cores::models::load_db().await;
    let result = enviroment::Environment::update_envirionment_status(2, 0).await;

    println!("{:?}", result);
}

#[tokio::test]
async fn test_environment_update_proxy() {
    use cores::models::enviroment::{self};

    crate::cores::models::load_db().await;
    let result = enviroment::Environment::update_envirionment_proxy(2, "").await;

    println!("{:?}", result);
}

#[tokio::test]
async fn test_environment_update_ua() {
    use cores::models::enviroment::{self};

    crate::cores::models::load_db().await;
    let result = enviroment::Environment::update_envirionment_ua(2, "abc").await;

    println!("{:?}", result);
}

#[tokio::test]
async fn test_environment_update_group() {
    use cores::models::enviroment::{self};

    crate::cores::models::load_db().await;
    let result = enviroment::Environment::update_envirionment_group(2, 3).await;

    println!("{:?}", result);
}

#[tokio::test]
async fn test_environment_update_fp() {
    use cores::models::enviroment::{self};

    crate::cores::models::load_db().await;
    let result = enviroment::Environment::update_envirionment_fp(2, 3).await;

    println!("{:?}", result);
}

#[tokio::test]
async fn test_environment_delete() {
    use cores::models::enviroment::{self};

    crate::cores::models::load_db().await;
    let result = enviroment::Environment::delete_envirionment(2).await;
    println!("{:?}", result);
}
