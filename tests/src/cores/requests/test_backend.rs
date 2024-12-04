mod enviroment {

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

        let msg = cores::requests::backend::auth::register(
            "lius",
            "lius",
            "liushuinew@163.com",
            "681397",
        )
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

    #[tokio::test]
    async fn test_get_environment_by_group_id() {
        use cores::apis::PageParam;

        cores::requests::backend::auth::login("lius", "lius")
            .await
            .unwrap();

        let data = cores::requests::backend::environment::get_environment_by_group_id(
            1,
            &PageParam {
                page_num: Some(1),
                page_size: Some(10),
            },
        )
        .await
        .unwrap();

        println!("{:?}", data);
    }

    #[tokio::test]
    async fn test_get_environment_by_id() {
        cores::requests::backend::auth::login("lius", "lius")
            .await
            .unwrap();

        let data = cores::requests::backend::environment::get_environment_by_id(13)
            .await
            .unwrap();

        println!("{:?}", data);
    }

    #[tokio::test]
    async fn test_get_environment_list() {
        use cores::apis::PageParam;
        let payload = PageParam {
            page_num: Some(1),
            page_size: Some(3),
        };

        cores::requests::backend::auth::login("lius", "lius")
            .await
            .unwrap();

        let data = cores::requests::backend::environment::get_environment_list(&payload)
            .await
            .unwrap();

        println!("{:?}", data);
    }

    #[tokio::test]
    async fn test_create_environment() {
        cores::requests::backend::auth::login("lius", "lius")
            .await
            .unwrap();

        let environ = cores::models::enviroment::Environment {
        id: Some(0),
        name: String::from("hello Environment"),
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

        let data = cores::requests::backend::environment::create_environment(environ)
            .await
            .unwrap();

        println!("{:?}", data);
    }

    #[tokio::test]
    async fn test_update_environment() {
        cores::requests::backend::auth::login("lius", "lius")
            .await
            .unwrap();

        let environ = cores::models::enviroment::Environment {
        id: Some(0),
        name: String::from("hsjdhfjsfh Environment"),
        description:None,
        owner_id: String::from("1"),
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

        let data = cores::requests::backend::environment::update_environment(13, environ)
            .await
            .unwrap();

        println!("{:?}", data);
    }

    #[tokio::test]
    async fn test_delete_environment() {
        cores::requests::backend::auth::login("lius", "lius")
            .await
            .unwrap();

        let data = cores::requests::backend::environment::delete_environment(13)
            .await
            .unwrap();

        println!("{:?}", data);
    }
}

mod group {

    #[tokio::test]
    async fn test_group_create() {
        cores::requests::backend::auth::login("lius", "lius")
            .await
            .unwrap();

        let data =
            cores::requests::backend::group::create_group("lius group", "this is lius group")
                .await
                .unwrap();

        println!("{:?}", data);
    }

    #[tokio::test]
    async fn test_delete_group() {
        cores::requests::backend::auth::login("lius", "lius")
            .await
            .unwrap();

        let data = cores::requests::backend::group::delete_group(2)
            .await
            .unwrap();

        println!("{:?}", data);
    }

    #[tokio::test]
    async fn test_update_group() {
        cores::requests::backend::auth::login("lius", "lius")
            .await
            .unwrap();

        let data = cores::requests::backend::group::update_group(3, "abc", "this")
            .await
            .unwrap();

        println!("{:?}", data);
    }

    #[tokio::test]
    async fn test_get_group_list() {
        use cores::apis::PageParam;

        cores::requests::backend::auth::login("lius", "lius")
            .await
            .unwrap();

        let data = cores::requests::backend::group::get_group_list(&PageParam {
            page_num: Some(0),
            page_size: Some(10),
        })
        .await
        .unwrap();

        println!("{:?}", data);
    }
}
