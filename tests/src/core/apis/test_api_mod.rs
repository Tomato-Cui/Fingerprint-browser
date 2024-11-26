#[test]
fn test_get_browser_list_handle() {
    use core::apis::browser;

    let browsers = browser::get_browser_list_handle();
    println!("{:?}", browsers)
}

#[test]
fn test_update_browser_handle() {
    use core::apis::browser;
    use core::models::browser::Browser;
    use core::models::browser::BrowserFp;

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
        h: 1080,                          // 屏幕高度
        w: 1920,                          // 屏幕宽度
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
        id: 1, // `id` 字段会被序列化，但在反序列化时会被跳过
        tags: "tag1,tag2".to_string(),
        group: "group1".to_string(),
        porxy: "proxy123".to_string(),
        ua: "Mozilla/5.0".to_string(),
        ipaddr: "192.168.1.1".to_string(),
        region: "US".to_string(),
        os: "Windows 10".to_string(),
        time: 1630000000,    // 使用合适的时间戳
        fp_info: browser_fp, // 引用前面创建的 `BrowserFp` 对象
        is_tz: true,
        is_pos: false,
        porxy_file: "proxy_file.txt".to_string(),
        user_data_file: "user_data.txt".to_string(),
        status: false,
    };

    let browsers = browser::update_browser_handle(browser, 1);
    println!("{:?}", browsers)
}

#[test]
fn test_delete_browser_handle() {
    use core::apis::browser;

    let browsers = browser::delete_browser_handle(1);
    println!("{:?}", browsers)
}

#[test]
fn test_add_browser_handle() {
    use core::apis::browser;
    use core::models::browser::Browser;
    use core::models::browser::BrowserFp;

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
        h: 1080,                          // 屏幕高度
        w: 1920,                          // 屏幕宽度
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
        id: 1, // `id` 字段会被序列化，但在反序列化时会被跳过
        tags: "tag1,tag2".to_string(),
        group: "group1".to_string(),
        porxy: "proxy123".to_string(),
        ua: "Mozilla/5.0".to_string(),
        ipaddr: "192.168.1.1".to_string(),
        region: "US".to_string(),
        os: "Windows 10".to_string(),
        time: 1630000000,    // 使用合适的时间戳
        fp_info: browser_fp, // 引用前面创建的 `BrowserFp` 对象
        is_tz: true,
        is_pos: false,
        porxy_file: "proxy_file.txt".to_string(),
        user_data_file: "user_data.txt".to_string(),
        status: false,
    };

    let browsers = browser::add_browser_handle(browser);
    println!("{:?}", browsers)
}
