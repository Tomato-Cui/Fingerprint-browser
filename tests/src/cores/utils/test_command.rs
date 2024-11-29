#[allow(dead_code)]
#[tokio::test]
async fn test_start_stop_browser() {
    use cores::models::enviroment::Browser;
    use cores::models::enviroment::BrowserFp;
    use cores::utils::command::BrowserChildInfo;
    use cores::utils::command::Processer;
    use cores::utils::common::get_debug_port;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::Mutex;

    crate::cores::init_config().await;

    let processer = Arc::new(Mutex::new(Processer::new()));

    let browser_fp = BrowserFp {
        os_mem: 8192, // 内存 8GB
        os_ver: "Windows 10".to_string(),
        proc_num: 8.0,                    // 8 个 CPU 逻辑处理器
        audio: 30,                        // 随机音频指纹值
        gl_ven: "NVIDIA".to_string(),     // GPU 厂商
        gl_rend: "OpenGL".to_string(),    // GPU 渲染引擎
        gl_rd: 0.25,                      // GPU 随机数
        cv_n: 0.35,                       // Canvas 随机数
        cv_s: "a1b2c3".to_string(),       // Canvas 随机字符
        c_r: 1.0000002,                   // ClientRects 指纹
        css: 5,                           // CSS 字体随机值
        h: 1080.0,                          // 屏幕高度
        w: 1920.0,                          // 屏幕宽度
        p: "80,22,443".to_string(),       // 端口扫描防护
        la: 37.7749,                      // 窗口位置纬度
        lo: -122.4194,                    // 窗口位置经度
        tz: "GMT+8".to_string(),          // 时区
        lang: "en-US".to_string(),        // 语言
        v_l: "0,1,2".to_string(),         // SpeechVoices (随机选 0, 1, 2)
        f_l: "3455,1500,200".to_string(), // 字体列表 (随机选取的字体)
        tag: "example_tag".to_string(),
        dl: "https://example.com/download".to_string(), // 下载链接
    };

    let browser = Browser {
        id: Some(1),
        name: Some(String::from("Test Browser")),
        domain_name: Some(vec![
            String::from("facebook.com"),
            String::from("amazon.com"),
        ]),
        open_urls: Some(vec![String::from("https://www.example.com")]),
        repeat_config: Some(0),
        username: Some(String::from("test_user")),
        password: Some(String::from("test_password")),
        fakey: Some(String::from("test_2FA_key")),
        cookie: Some(String::from("{\"name\":\"value\"}")),
        ignore_cookie_error: Some(1),
        tags: Some(vec![String::from("tag1"), String::from("tag2")]),
        group_id: Some(String::from("group_1")),
        ua: String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36"),
        os: String::from("Windows 10"),
        country: Some(String::from("US")),
        region: Some(String::from("California")),
        city: Some(String::from("San Francisco")),
        remark: Some(String::from("Test Browser instance")),
        ipchecker: Some(String::from("ipapi")),
        sys_app_cate_id: Some(1),
        user_proxy_config: Some(String::from("{\"proxy\":\"proxy_config\"}")),
        proxy: Some(String::from("192.168.1.1")),
        proxy_enable: true,
        fingerprint_config: String::from("{\"fingerprint\":\"unique_fingerprint_value\"}"),
        created_at: 1633072800, // Unix 时间戳
        fp_info: browser_fp,
        is_tz: true,
        is_pos: true,
        user_data_file: String::from("path/to/user_data_file"),
        status: true,
    };

    let port = get_debug_port().await.unwrap();
    // let proxy = get_proxy_from_registry().unwrap();

    let payload = BrowserChildInfo::new(
        browser,
        port,
        "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
    );
    processer.lock().await.start_browser(payload).await.unwrap();

    tokio::time::sleep(Duration::from_secs(5)).await;

    let _ = processer.lock().await.status(1).await.unwrap();
    // for _ in 0..100 {
    //     let status = processer.lock().await.status(1).await.unwrap();
    //     println!("{:?}", status);
    // }
}
