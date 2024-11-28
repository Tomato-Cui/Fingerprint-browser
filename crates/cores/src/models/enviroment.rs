/// 浏览器的数据结构
use crate::{
    apis::PageParam,
    config::get_config,
    db::Db,
    errors::ApplicationServerError,
    utils::common::{option_vec_string_to_string, string_to_option_vec_string},
};
use rusqlite::params;
use serde::{Deserialize, Serialize};

/// 浏览器指纹数据结构
#[derive(Debug, Deserialize, Serialize)]
pub struct BrowserFp {
    pub os_mem: i32,     //内存
    pub os_ver: String,  //系统版本
    pub proc_num: f64,   //cpu 逻辑处理器数量
    pub audio: i32,      //音频指纹  0-49 之间的整数
    pub gl_ven: String,  //gpu 厂商(linux下nvidia显卡有 Corporation 字符串 不清楚其他品牌有没有)
    pub gl_rend: String, //gpu 渲染引擎
    pub gl_rd: f32,      //gpu 随机数 0-0.5 之间的浮点数
    pub cv_n: f32,       //canvas 随机 0-0.5 的浮点数
    pub cv_s: String,    //canvas 随机 a-f 的字符
    pub c_r: f64,        //ClientRects 指纹  1.0000001 - 1.0000003
    pub css: i32,        //css 字体  随机 0-19
    pub h: i32,          //屏幕高度
    pub w: i32,          //屏幕宽度
    pub p: String,       //端口扫描防护 例子: 0 或 80,22,443
    pub la: f64,         //窗口位置纬度
    pub lo: f64,         //窗口位置经度
    pub tz: String,      //时区
    pub lang: String,    //语言
    pub v_l: String,     //SpeechVoices win: 0-74 随机3个 mac: 0-118 随机48个 linux: 空
    pub f_l: String,     //字体列表 生成 0-3455 的随机450-1500个不重复的值
    pub tag: String,
    pub dl: String,
}

//定义浏览器环境数据结构
#[derive(Debug, Deserialize, Serialize)]
pub struct Browser {
    pub id: Option<i8>,
    pub name: Option<String>,             // 环境名
    pub domain_name: Option<Vec<String>>, // 账号平台的域名：facebook.com, amazon.com...会在打开浏览器时默认访问
    pub open_urls: Option<Vec<String>>, // 浏览器打开时访问的其他url地址，不填则默认只打开domain_name的地址
    pub repeat_config: Option<i32>, // 账号去重，默认允许重复，支持0允许重复；2根据账密去重；3根据cookie去重；4根据c_user去重(c_user是FaceBook专有标记)
    pub username: Option<String>,   // 账号密码或者Cookie至少填一个；账号允许重复则都可不填
    pub password: Option<String>,   // 账号密码或者Cookie至少填一个；账号允许重复则都可不填
    pub fakey: Option<String>,      // 填写2FA密钥。适用于网站的二次验证码生成，类似Google身份验证器
    pub cookie: Option<String>, // 账号密码或者Cookie至少填一个；账号允许重复则都可不填；支持JSON和Netscape格式
    pub ignore_cookie_error: Option<i32>, // 0：校验cookie失败时，直接返回cookie格式不正确；1：校验cookie失败时，过滤掉格式错误的数据，保留正确格式的cookie，仅支持Netscape格式
    pub tags: Option<Vec<String>>,        // 添加到对应标签ID，未分配标签则可以传0
    pub group_id: Option<String>,         // 添加到对应分组的ID，未分配分组则可以传0
    pub ua: String,                       // user agent
    pub os: String,
    pub country: Option<String>, // 环境使用的代理国家/地区，lumauto、oxylabs如果没有IP则需要填写国家
    pub region: Option<String>,  // 环境使用的代理州/省，可不填
    pub city: Option<String>,    // 环境使用的代理城市，可不填
    pub remark: Option<String>,  // 备注
    pub ipchecker: Option<String>, // IP查询渠道，支持传入ip2location、ipapi
    pub sys_app_cate_id: Option<i32>, // 可传入应用分类ID，0为跟随团队应用
    pub user_proxy_config: Option<String>, // 环境代理配置，具体查看参数对象userProxyConfig
    pub proxy: Option<String>,   // 环境使用的代理IP，代理软件为lumauto、oxylabs填写
    pub proxy_enable: bool,      //  代理是否启用
    pub fingerprint_config: String, // 指纹配置，具体查看参数对象fingerprintConfig
    pub created_at: i64,
    pub fp_info: BrowserFp,
    pub is_tz: bool,
    pub is_pos: bool,
    pub user_data_file: String,
    pub status: bool, // 浏览器状态
    pub lang: Option<String>,
}

impl Browser {
    /// browser表插入数据
    pub fn insert_browser(browser: Browser) -> Result<bool, ApplicationServerError> {
        let db = Db::new(get_config()?.get_cache_location()?)?;

        let sql = "
        INSERT INTO environments (
            name, 
            domain_name, 
            open_urls, 
            repeat_config, 
            username, 
            password, 
            fakey, 
            cookie, 
            ignore_cookie_error, 
            tags, 
            group_id, 
            ua, 
            os, 
            country, 
            region, 
            city, 
            remark, 
            ipchecker, 
            sys_app_cate_id, 
            user_proxy_config, 
            proxy, 
            proxy_enable, 
            fingerprint_config, 
            created_at, 
            fp_info, 
            is_tz, 
            is_pos, 
            user_data_file, 
            status,
            lang 
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30)";

        let status = db.query_table(
            sql,
            params![
                &browser.name,
                option_vec_string_to_string(browser.domain_name, ","),
                option_vec_string_to_string(browser.open_urls, " "),
                browser.repeat_config.unwrap_or(0),
                browser.username.unwrap_or_default(),
                browser.password.unwrap_or_default(),
                browser.fakey.unwrap_or_default(),
                browser.cookie.unwrap_or_default(),
                browser.ignore_cookie_error.unwrap_or(0),
                option_vec_string_to_string(browser.tags, ","),
                browser.group_id.unwrap_or_default(),
                browser.ua,
                browser.os,
                browser.country.unwrap_or_default(),
                browser.region.unwrap_or_default(),
                browser.city.unwrap_or_default(),
                browser.remark.unwrap_or_default(),
                browser.ipchecker.unwrap_or_default(),
                browser.sys_app_cate_id.unwrap_or(0),
                browser.user_proxy_config.unwrap_or_default(),
                browser.proxy.unwrap_or_default(),
                browser.proxy_enable,
                browser.fingerprint_config,
                browser.created_at,
                serde_json::to_string(&browser.fp_info)?,
                browser.is_tz,  // Convert bool to integer (1 or 0)
                browser.is_pos, // Convert bool to integer (1 or 0)
                browser.user_data_file,
                browser.status,
                browser.lang.unwrap_or_default()
            ],
        )?;
        if status == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// browser表删除数据
    pub fn delete_browser(ids: Vec<u8>) -> Result<bool, ApplicationServerError> {
        let db = Db::new(get_config()?.get_cache_location()?)?;

        let delete_ids = ids
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<String>>()
            .join(",");

        let sql = format!("delete from environments where id in ({})", delete_ids);
        let status = db.query_table(&sql, params![])?;
        if status == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// browser表查询所有数据
    pub fn query_browser_by_id(id: i8) -> Result<Option<Browser>, ApplicationServerError> {
        let db = Db::new(get_config()?.get_cache_location()?)?;

        let sql = &format!("select * from environments where id = ({})", id);
        let mut stmt = db.query_map_table(sql)?;
        let browser_iter = stmt.query_map(params![], |row| {
            let str_value: String = row.get(25)?;
            let browser_fp: BrowserFp = serde_json::from_str(&str_value).unwrap();

            Ok(Browser {
                id: row.get(0)?,
                name: row.get(1)?,
                domain_name: string_to_option_vec_string(row.get(2)?, ","),
                open_urls: string_to_option_vec_string(row.get(3)?, " "),
                repeat_config: row.get(4)?,
                username: row.get(5)?,
                password: row.get(6)?,
                fakey: row.get(7)?,
                cookie: row.get(8)?,
                ignore_cookie_error: row.get(9)?,
                tags: string_to_option_vec_string(row.get(10)?, ","),
                group_id: row.get(11)?,
                ua: row.get(12)?,
                os: row.get(13)?,
                country: row.get(14)?,
                region: row.get(15)?,
                city: row.get(16)?,
                remark: row.get(17)?,
                ipchecker: row.get(18)?,
                sys_app_cate_id: row.get(19)?,
                user_proxy_config: row.get(20)?,
                proxy: row.get(21)?,
                proxy_enable: row.get(22)?,
                fingerprint_config: row.get(23)?,
                created_at: row.get(24)?,
                fp_info: browser_fp,
                is_tz: row.get(26)?,
                is_pos: row.get(27)?,
                user_data_file: row.get(28)?,
                status: row.get(29)?,
                lang: row.get(30)?,
            })
        })?;

        for b in browser_iter {
            if let Ok(browser) = b {
                return Ok(Some(browser));
            }
        }
        Ok(None)
    }

    /// browser表查询所有数据
    pub fn query_browser(payload: PageParam) -> Result<Vec<Browser>, ApplicationServerError> {
        let db = Db::new(get_config()?.get_cache_location()?)?;

        let page_num = payload.page_num.unwrap_or_else(|| 1);
        let page_size = payload.page_size.unwrap_or_else(|| 10);
        let offset = page_num * page_size;

        let sql = &format!(
            "select * from environments limit ({}) offset ({})",
            page_size, offset
        );

        let mut stmt = db.query_map_table(sql)?;
        let browser_iter = stmt.query_map(params![], |row| {
            let str_value: String = row.get(25)?;
            let browser_fp: BrowserFp = serde_json::from_str(&str_value).unwrap();

            Ok(Browser {
                id: row.get(0)?,
                name: row.get(1)?,
                domain_name: string_to_option_vec_string(row.get(2)?, ","),
                open_urls: string_to_option_vec_string(row.get(3)?, " "),
                repeat_config: row.get(4)?,
                username: row.get(5)?,
                password: row.get(6)?,
                fakey: row.get(7)?,
                cookie: row.get(8)?,
                ignore_cookie_error: row.get(9)?,
                tags: string_to_option_vec_string(row.get(10)?, ","),
                group_id: row.get(11)?,
                ua: row.get(12)?,
                os: row.get(13)?,
                country: row.get(14)?,
                region: row.get(15)?,
                city: row.get(16)?,
                remark: row.get(17)?,
                ipchecker: row.get(18)?,
                sys_app_cate_id: row.get(19)?,
                user_proxy_config: row.get(20)?,
                proxy: row.get(21)?,
                proxy_enable: row.get(22)?,
                fingerprint_config: row.get(23)?,
                created_at: row.get(24)?,
                fp_info: browser_fp,
                is_tz: row.get(26)?,
                is_pos: row.get(27)?,
                user_data_file: row.get(28)?,
                status: row.get(29)?,
                lang: row.get(30)?,
            })
        })?;

        let browser: Vec<Browser> = browser_iter.filter_map(Result::ok).collect();
        Ok(browser)
    }

    /// browser表查询所有数据
    /// 更新数据 由于更新的数据有好几个 为了简单索性更新全部数据 (根据id来更新数据)
    pub fn update_browser(browser: Browser) -> Result<bool, ApplicationServerError> {
        let db = Db::new(get_config()?.get_cache_location()?)?;

        let sql = "
            UPDATE environments
            SET name = ?1,
                domain_name = ?2,
                open_urls = ?3,
                repeat_config = ?4,
                username = ?5,
                password = ?6,
                fakey = ?7,
                cookie = ?8,
                ignore_cookie_error = ?9,
                tags = ?10,
                group_id = ?11,
                ua = ?12,
                os = ?13,
                country = ?14,
                region = ?15,
                city = ?16,
                remark = ?17,
                ipchecker = ?18,
                sys_app_cate_id = ?19,
                user_proxy_config = ?20,
                proxy = ?21,
                proxy_enable = ?22,
                fingerprint_config = ?23,
                created_at = ?24,
                fp_info = ?25,
                is_tz = ?26,
                is_pos = ?27,
                user_data_file = ?28,
                status = ?29,
                lang = ?30
            WHERE id = ?31
        ";

        let status = db.query_table(
            sql,
            params![
                &browser.name,
                option_vec_string_to_string(browser.domain_name, ","),
                option_vec_string_to_string(browser.open_urls, " "),
                browser.repeat_config.unwrap_or(0),
                browser.username.unwrap_or_default(),
                browser.password.unwrap_or_default(),
                browser.fakey.unwrap_or_default(),
                browser.cookie.unwrap_or_default(),
                browser.ignore_cookie_error.unwrap_or(0),
                option_vec_string_to_string(browser.tags, ","),
                browser.group_id.unwrap_or_default(),
                browser.ua,
                browser.os,
                browser.country.unwrap_or_default(),
                browser.region.unwrap_or_default(),
                browser.city.unwrap_or_default(),
                browser.remark.unwrap_or_default(),
                browser.ipchecker.unwrap_or_default(),
                browser.sys_app_cate_id.unwrap_or(0),
                browser.user_proxy_config.unwrap_or_default(),
                browser.proxy.unwrap_or_default(),
                browser.proxy_enable,
                browser.fingerprint_config,
                browser.created_at,
                serde_json::to_string(&browser.fp_info)?,
                browser.is_tz,  // Convert bool to integer (1 or 0)
                browser.is_pos, // Convert bool to integer (1 or 0)
                browser.user_data_file,
                browser.status,
                browser.lang.unwrap_or_default(),
                browser.id
            ],
        )?;
        if status == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// browser表更新浏览器状态
    ///
    /// 可以通过这个方法来更新浏览器的启动和关闭
    pub fn update_browser_status(id: i8, status: bool) -> Result<bool, ApplicationServerError> {
        let db = Db::new(get_config()?.get_cache_location()?)?;

        let sql = "
            UPDATE environments 
            SET status = ?1
            WHERE id = ?2
        ";
        let status = db.query_table(sql, params![status, id])?;
        if status == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// browser表更新浏览器代理
    pub fn update_browser_proxy(
        id: i8,
        proxy: Option<&str>,
    ) -> Result<bool, ApplicationServerError> {
        let db = Db::new(get_config()?.get_cache_location()?)?;

        let sql = "
            UPDATE environments 
            SET proxy = ?1
            WHERE id = ?2
        ";
        let affact = db.query_table(sql, params![proxy.unwrap_or(""), id])?;
        if affact == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// browser表更新浏览器ua
    ///
    /// empty ua 已经在函数外被筛选
    pub fn update_browser_ua(id: i8, ua: Option<&str>) -> Result<bool, ApplicationServerError> {
        let db = Db::new(get_config()?.get_cache_location()?)?;

        let sql = "
            UPDATE environments 
            SET proxy = ?1
            WHERE id = ?2
        ";

        let affact = db.query_table(sql, params![ua.unwrap_or_default(), id])?;
        if affact == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn update_browser_group(id: u8, group_id: u8) -> Result<bool, ApplicationServerError> {
        let db = Db::new(get_config()?.get_cache_location()?)?;

        let sql = "
            UPDATE environments 
            SET group_id = ?1
            WHERE id = ?2
        ";

        let affact = db.query_table(sql, params![group_id, id])?;
        if affact == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn get_browser_unique(&self) -> String {
        format!(
            "{}.{}.{}",
            self.id.unwrap_or_default(),
            self.os,
            self.created_at
        )
    }
}
