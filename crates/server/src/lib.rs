use axum::{http::StatusCode, middleware, routing::get, Router};

pub mod middlewares;
pub mod response;
pub mod routes;

/// resource is not found
/// 第二各返回值为空，由浏览器默认内容填充就好了。
async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "")
}

pub fn build_root_router() -> Router {
    Router::new()
        .nest(
            "/api/management/v1",
            Router::new()
                .merge(routes::browser::build_browser_router())
                .merge(routes::user::build_router())
                .merge(routes::environment::build_environment_router())
                .merge(routes::group::build_group_router())
                .merge(
                    Router::new()
                        .route("/status", get(|| async { String::from("status: running") })),
                ),
        )
        .route_layer(middleware::from_fn(middlewares::logger))
        .fallback(not_found)
}

/// fingerprint_config对象是浏览器指纹配置的参数信息，支持多种浏览器指纹配置是AdsPower的产品特性之一。
///
/// random_ua
/// ua_version: list，非必填，支持当前主流版本，不填默认在所有版本中随机。
///
/// ua_system_version: list，非必填，不填默认在所有系统中随机，支持 ：
///
/// Android（*指定版本：Android 9、Android 10、Android 11、Android 12、Android 13）
/// iOS（*指定版本：iOS 14、iOS 15）
/// Windows（*指定版本：Windows 7、Windows 8、Windows 10、Windows 11）
/// Mac OS X（*指定版本：Mac OS X 10、Mac OS X 11、Mac OS X 12、Mac OS X 13）
/// Linux
/// fingerprint_config需要传入对应的JSON对象，不能为{}，示例:
pub struct FingerPrintConfig {
    pub automatic_timezone: bool, //     automatic_timezone	text	否	1	1	1：基于IP自动生成对应的时区(默认)； 0：指定时区。
    pub timezone: String, // timezone	text	否	-	America/Yellowknife	指定时区，默认空字符串""代表本地时区。
    pub webrtc: String,   // webrtc	text	否	disabled	disabled	Chrome即时通信组件，支持：
    pub forward: String, // forward 转发，使用代理IP覆盖真实IP，代理场景使用，更安全（需升级到V2.6.8.6及以上版本 ）；
    pub proxy: String,   // proxy 替换 ，使用代理IP覆盖真实IP，代理场景使用
    pub local: String,   // local 真实 ，网站会获取真实IP；
    pub disabled: String, // disabled 禁用(默认)，网站会拿不到IP。
    pub location: String, // location	text	否	ask	ask	网站请求获取您当前地理位置时的选择，支持：询问ask(默认)，与普通浏览器的提示一样；允许allow，始终允许网站获取位置；禁止block，始终禁止网站获取位置。
    pub location_switch: String, // location_switch	text	否	1	1	1：基于IP自动生成对应的位置(默认)； 0：指定位置。
    pub longitude: String, // longitude	text	否	-	-40.123321	指定位置的经度，指定位置时必填，范围是-180到180，支持小数点后六位。
    pub latitude: String, // latitude	text	否	-	30.123321	指定位置的纬度，指定位置时必填，范围是-90到90，支持小数点后六位。
    pub accuracy: String, // accuracy	text	否	1000	1000	指定位置的精度(米) ，指定位置时必填，范围10-5000，整数。
    pub language: Vec<String>, // language	list	否	["en-US","en"]	["en-US","en","zh-CN","zh"]	浏览器的语言(默认["en-US","en"])，支持传多个语言，格式为字符串数组。
    pub language_switch: String, // language_switch	text	否	1	1	基于IP国家设置语言：0：关闭；1：启用。	需升级到V2.4.4.3及以上版本。
    pub page_language_switch: String, // page_language_switch	text	否	1	1	基于[语言]匹配界面语言， 0：关闭； 1：启用。	使用条件： 1. 需要升级至 “Patch 2.6.7.2” 及以上； 2. 支持的 SunBrowser 版本： a. Windows: Chrome 109 及以上的版本。 b. macOS：Chrome 119 以上的版本。
    pub page_language: String, // page_language	text	否	native	en-US	page_language_switch需为0才生效，page_language默认为native，即本地语言，也可传入国家code，具体查看界面语言
    pub ua: String, // ua	text	否	-	Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.113 Safari/537.36	user-agent用户信息，默认不传使用随机ua库， 自定义需要确保ua格式与内容符合标准。
    pub screen_resolution: String, // screen_resolution	text	否	none	1024_600	屏幕分辨率，none: 使用电脑当前分辨率; random: 随机分辨率; 自定义需要下划线分隔，宽_高。
    pub fonts: Vec<String>, // fonts	list	否	-	["all"] // ["Arial","Calibri","Cambria"]	浏览器的字体(默认所有) // 自定义支持多字体英文，格式为字符串数组。
    pub canvas: String, // canvas	text	否	1	1	浏览器canvas指纹开关 1：添加噪音(默认)； 0：电脑默认。
    pub webgl_image: String, // webgl_image	text	否	1	1	浏览器webgl图像指纹开关// 1：添加噪音(默认)； // 0：电脑默认。
    pub webgl: String, // webgl	text	否	3	3	浏览器webgl元数据指纹开关// 0：电脑默认；// 2：自定义（需定义webgl_config）；// 3：随机匹配(该类型仅在新建浏览器接口支持，更新环境信息接口暂不支持)。// 自定义，需升级到V2.4.3.9及以上版本。
    pub webgl_config: String,
    // webgl_config	json	否	{"unmasked_vendor": "", "unmasked_renderer": "", "webgpu": { "webgpu_switch": "1" }}	{"unmasked_vendor": "Google Inc.", "unmasked_renderer": "ANGLE (Intel(R) HD Graphics 620 Direct3D11 vs_5_0 ps_5_0)", "webgpu": { "webgpu_switch": "1" }}	浏览器webgl元数据自定义，unmasked_vendor：厂商，unmasked_renderer：渲染。
    // 该值只有在webgl为2时才会启动自定义。
    // 当webgl为2时，厂商和渲染均不能为空，否则采用电脑默认。
    // webgpu基于webgl_config:
    // 1：基于 WebGL 匹配；
    // 2：真实；
    // 0：禁用。
    // 需升级到V2.6.8.1及以上版本。
    pub audio: String, // audio	text	否	1	1	音频指纹开关：
    // 1：添加噪音(默认)；
    // 0：电脑默认。
    pub do_not_track: String,
    // do_not_track	text	否	default	true	DNT即"do not track"，“请勿跟踪”浏览器设置开关，支持：
    // default(默认)；
    // true：开启；
    // false：关闭。
    pub hardware_concurrency: String,
    // hardware_concurrency	text	否	4	4	电脑CPU核数，支持：
    // default(电脑实际CPU核数)，2，4(不传默认4核)，6，8，16。
    pub device_memory: String,
    // device_memory	text	否	8	8	电脑内存大小，支持：
    // default(电脑实际内存大小)， 2，4，6，8(不传默认8G)。
    pub flash: String,
    // flash	text	否	block	allow	flash配置开关，支持：
    // allow：启用；
    // block：关闭(默认)。
    pub scan_port_type: String,
    // scan_port_type	text	否	1	1	端口扫描保护，支持：
    // 1：启用(默认)；
    // 0：关闭。
    pub allow_scan_ports: Vec<String>,
    // allow_scan_ports	list	否	-	["4000","4001"]	端口扫描保护启用时允许被扫描的指定端口，格式为字符串数组，默认不传为空。
    pub media_devices: String,
    // media_devices	text	否	1	1	媒体设备开关：
    // 0：关闭（每个
    // 浏览器使用当前电脑默认的媒体设备id）；
    // 1:  噪音（设备数量跟随本机）；
    // 2：噪音（自定义设备数量，需传 media_devices_num）。	需升级到V2.6.4.2及以上版本 。
    pub media_devices_num: String,
    // media_devices_num	text	否	{"audioinput_num": "1", "videoinput_num": "1", "audiooutput_num": "1"}	{"audioinput_num": "1", "videoinput_num": "2", "audiooutput_num": "3"}	audioinput_num： 麦克风数量(1-9)；
    pub videoinput_num: String,
    // videoinput_num：摄像机数量(1-9)；
    pub audiooutput_num: String,
    // audiooutput_num: 扬声器数量(1-9)	需升级到V2.6.4.2及以上版本。
    pub client_rects: String,
    // client_rects	text	否	1	1	ClientRects指纹：
    // 0：每个浏览器使用当前电脑默认的ClientRects；
    // 1：添加相应的噪音，同一电脑上为每个浏览器生成不同的ClientRects。	需升级到V3.6.2及以上版本。
    pub device_name_switch: String,
    // device_name_switch	text	否	1	1	设备名称：
    // 0：关闭, 每个浏览器使用当前电脑的设备名称；
    // 1：掩盖, 使用合适的值代替您真实的设备名称；
    // 2：自定义设备名称。
    // 需升级到3.6.25及以上版本，值为2时需升级到V2.4.8.1及以上版本。
    pub device_name: String,
    // device_name	text	否	-	abcd	自定义设备名称。	需升级到V2.4.8.1及以上版本。
    pub random_ua: String,
    // random_ua	json	否	-	{"ua_browser":["chrome"],"ua_version":["80"],"ua_system_version":["Windows 10"]}	支持指定类型、系统、版本设置ua。若同时传入了自定义ua，则优先使用自定义的ua。
    // ua_browser: 类型，chrome || firefox；
    // ua_system_version: 系统；
    // ua_version: 版本;
    // 该字段仅在新建浏览器接口支持，更新环境接口暂不支持指定类型、系统、版本更新ua。
    // 详情见random_ua	建议：ua_browser与browser_kernel_config下的type一致，默认chrome。
    pub speech_switch: String,
    // speech_switch	text	否	1	1	SpeechVoices指纹：
    // 0：每个浏览器使用当前电脑默认的SpeechVoices；
    // 1：添加相应的噪音，同一电脑上为每个浏览器生成不同的SpeechVoices。	软件版本需升级到V3.11.10及以上版本,内核版本需升级到V2.5.0.9及以上版本。
    pub mac_address_config: String,
    // mac_address_config	json	否	{"model": "1", "address": ""}	{"model": "2", "address": "E4-02-9B-3B-E9-27"}	MAC地址：支持设置合适的值代替真是的MAC地址。
    // model: 0 （使用当前电脑的MAC地址），1（匹配合适的值代替真实的MAC地址）， 2（自定义合适的值代替真实的MAC地址） 。
    // address: 自定义MAC地址，当model为2时，需传入该值。	软件版本需升级到V4.3.9及以上版本。
    pub browser_kernel_config: String,
    // browser_kernel_config	json	否	{"version": "ua_auto", "type":"chrome"}	{"version": "99", "type":"chrome"}	使用对应浏览器内核打开浏览器。
    // version:内核版本，参数说明："92"为92版内核、"99"为99版内核；"ua_auto"为智能匹配；
    // type：浏览器类型，chrome || firefox。	软件版本需升级到v4.4.21及以上版本。该版本仅支持version的可选值为'92'、'99'、'102'、'105'、'108'、'111'。firefox仅支持100内核版本；
    // window7仅支持108及以下内核版本。
    pub gpu: String,
    // gpu	text	否	0	0	0：使用【本地设置-硬件加速】的配置；
    // 1：开启硬件加速，可提升浏览器性能。使用不同的硬件，可能会影响硬件相关的指纹；
    // 2：关闭硬件加速，会降低浏览器性能。
}

/// user_proxy_config对象是环境代理配置的参数信息，AdsPower支持市面上常用的代理软件和协议。
///
/// user_proxy_config需要传入对应的JSON对象，示例：
/// 代理商	JSON	说明
/// brightdata	{"proxy_soft":"brightdata","proxy_type":"http","proxy_host":"xxxx",
/// "proxy_port":"xx","proxy_user":"xxx","proxy_password":"**"}	proxy_type支持设置使用http、https、socks5
/// brightauto	{"proxy_soft":"brightauto","proxy_type":"http","proxy_host":"xxxx",
/// "proxy_port":"xx","proxy_user":"xxx","proxy_password":"**"}	-
/// oxylabsauto	{"proxy_soft":"oxylabsauto","proxy_type":"http","proxy_host":"xxxx",
/// "proxy_port":"xx","proxy_user":"xx","proxy_password":"**"}	-
/// ipideaauto	{"proxy_soft":"ipideaauto","proxy_type":"http","proxy_host":"xxxx",
/// "proxy_port":"xx","proxy_user":"xx","proxy_password":"**"}	proxy_type支持设置使用http、socks5
/// ipfoxyauto	{"proxy_soft":"ipfoxyauto","proxy_type":"http","proxy_host":"xxxx",
/// "proxy_port":"xx","proxy_user":"xx","proxy_password":"**"}	proxy_type支持设置使用http、socks5
/// 922S5auth	{"proxy_soft":"922S5auth","proxy_type":"http","proxy_host":"xxxx",
/// "proxy_port":"xx","proxy_user":"xx","proxy_password":"**"}	-
/// kookauto	{"proxy_soft":"kookauto","proxy_type":"http","proxy_host":"xxxx",
/// "proxy_port":"xx","proxy_user":"xx","proxy_password":"**"}	proxy_type支持设置使用http、socks5
/// 922S5auto	{"proxy_soft":"922S5auto"}	使用922S5auto，只需传proxy_soft的值
/// other	{"proxy_soft":"other","proxy_type":"socks5","proxy_host":"xxxx",
/// "proxy_port":"xx","proxy_user":"xxx","proxy_password":"**"}	proxy_type支持设置使用http、https、socks5
/// no_proxy	{"proxy_soft":"no_proxy"}	-
pub struct UserProxyConfig {
    pub proxy_soft: String,
    // proxy_soft	text	是	-	brightdata	目前支持的代理有brightdata，brightauto，oxylabsauto，922S5auto，ipideaauto，ipfoxyauto，922S5auth，kookauto，ssh，other，no_proxy。
    pub proxy_type: String,
    // proxy_type	text	否	-	socks5	代理的类型，目前支持的类型有http，https，socks5；no_proxy可不传。
    pub proxy_host: String,
    // proxy_host	text	否	-	pr.oxylabs.io	代理服务器的地址，可以填域名或者IP；no_proxy可不传。
    pub proxy_port: String,
    //proxy_port	text	否	-	123	代理服务器的端口号；no_proxy可不传。
    pub proxy_user: String,
    // proxy_user	text	否	-	abc	代理需要登录时的账号。
    pub proxy_password: String,
    // proxy_password	text	否	-	xyz	代理需要登录时的密码。
    pub proxy_url: String,
    // proxy_url	text	否	-	http://www.xxx.com/	该URL用于移动代理，仅支持http/https/socks5的代理。
    // 1、通过该链接，您可以通过手动操作去改变代理的IP地址。
    // 2、多个环境使用同个代理账号时，刷新IP会改变同个代理账号的IP地址。
    pub global_config: String, // pub global_config	:text	否	0	1	使用代理管理的账号列表信息
}
