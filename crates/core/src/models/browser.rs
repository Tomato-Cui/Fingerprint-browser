/// 浏览器的数据结构
use crate::{config::AConfig, db::Db, errors::ApplicationServerError};
use rusqlite::params;
use serde::{Deserialize, Serialize};

/// 浏览器指纹数据结构
#[derive(Debug, Deserialize, Serialize)]
pub struct BrowserFp {
    os_mem: i32,     //内存
    os_ver: String,  //系统版本
    proc_num: f64,   //cpu 逻辑处理器数量
    audio: i32,      //音频指纹  0-49 之间的整数
    gl_ven: String,  //gpu 厂商(linux下nvidia显卡有 Corporation 字符串 不清楚其他品牌有没有)
    gl_rend: String, //gpu 渲染引擎
    gl_rd: f32,      //gpu 随机数 0-0.5 之间的浮点数
    cv_n: f32,       //canvas 随机 0-0.5 的浮点数
    cv_s: String,    //canvas 随机 a-f 的字符
    c_r: f64,        //ClientRects 指纹  1.0000001 - 1.0000003
    css: i32,        //css 字体  随机 0-19
    h: i32,          //屏幕高度
    w: i32,          //屏幕宽度
    p: String,       //端口扫描防护 例子: 0 或 80,22,443
    la: f64,         //窗口位置纬度
    lo: f64,         //窗口位置经度
    tz: String,      //时区
    lang: String,    //语言
    v_l: String,     //SpeechVoices win: 0-74 随机3个 mac: 0-118 随机48个 linux: 空
    f_l: String,     //字体列表 生成 0-3455 的随机450-1500个不重复的值
    tag: String,
    dl: String,
}

//定义浏览器环境数据结构
#[derive(Debug, Deserialize, Serialize)]
pub struct Browser {
    #[serde(skip_deserializing)] // 在反序列化时忽略该字段
    pub id: i32,
    pub tags: String,
    pub group: String,
    pub porxy: String,
    pub ua: String,
    pub ipaddr: String,
    pub region: String,
    pub os: String,
    pub time: i64,
    pub fp_info: BrowserFp,
    #[serde(rename = "isTz")]
    pub is_tz: bool,
    #[serde(rename = "isPos")]
    pub is_pos: bool,
    pub porxy_file: String,
    pub user_data_file: String,
}

impl Browser {
    /// browser表插入数据
    pub fn insert_browser(browser: &str) -> Result<bool, ApplicationServerError> {
        let browser_info: Browser = serde_json::from_str(browser)?;
        let db = Db::new(AConfig.get_cache_location())?;

        let sql = "insert into browser_table (tags, groups, porxy, ua, ipaddr, region, os, time, fp_info, isTz, isPos, porxy_file, user_data_file) values(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)";
        let status = db.query_table(
            sql,
            params![
                browser_info.tags,
                browser_info.group,
                browser_info.porxy,
                browser_info.ua,
                browser_info.ipaddr,
                browser_info.region,
                browser_info.os,
                browser_info.time,
                serde_json::to_string(&browser_info.fp_info)?,
                browser_info.is_tz,
                browser_info.is_pos,
                browser_info.porxy_file,
                browser_info.user_data_file,
            ],
        )?;
        if status == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// browser表删除数据
    pub fn delete_browser(id: i32) -> Result<bool, ApplicationServerError> {
        let db = Db::new(AConfig.get_cache_location())?;
        let sql = "delete from browser_table where id = (?1)";
        let status = db.query_table(sql, params![id])?;
        if status == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// browser表查询所有数据
    pub fn query_browser() -> Result<String, ApplicationServerError> {
        let db = Db::new(AConfig.get_cache_location())?;
        let sql = "select * from browser_table";
        let mut stmt = db.query_map_table(sql)?;
        let browser_iter = stmt.query_map(params![], |row| {
            let str_value: String = row.get(9)?;
            let browser_fp: BrowserFp = serde_json::from_str(&str_value).unwrap();
            Ok(Browser {
                id: row.get(0)?, // 索引 0
                tags: row.get(1)?,
                group: row.get(2)?,
                porxy: row.get(3)?,
                ua: row.get(4)?,
                ipaddr: row.get(5)?,
                region: row.get(6)?,
                os: row.get(7)?,
                time: row.get(8)?,
                fp_info: browser_fp,
                is_tz: row.get(10)?,
                is_pos: row.get(11)?,
                porxy_file: row.get(12)?,
                user_data_file: row.get(13)?,
            })
        })?;

        let browser: Vec<Browser> = browser_iter.filter_map(Result::ok).collect();
        Ok(serde_json::to_string(&browser)?)
    }

    /// browser表查询所有数据
    /// 更新数据 由于更新的数据有好几个 为了简单索性更新全部数据 (根据id来更新数据)
    pub fn update_browser(browser: &str, id: i32) -> Result<bool, ApplicationServerError> {
        let browser_info: Browser = serde_json::from_str(browser)?;
        let db = Db::new(AConfig.get_cache_location())?;

        let sql = "
        UPDATE browser_table
        SET tags = ?1,
            groups = ?2,
            porxy = ?3,
            ua = ?4,
            ipaddr = ?5,
            region = ?6,
            os = ?7,
            time = ?8,
            fp_info = ?9,
            isTz = ?10,
            isPos = ?11,
            porxy_file = ?12,
            user_data_file = ?13
        WHERE id = ?14
    ";
        let status = db.query_table(
            sql,
            params![
                browser_info.tags,
                browser_info.group,
                browser_info.porxy,
                browser_info.ua,
                browser_info.ipaddr,
                browser_info.region,
                browser_info.os,
                browser_info.time,
                serde_json::to_string(&browser_info.fp_info)?,
                browser_info.is_tz,
                browser_info.is_pos,
                browser_info.porxy_file,
                browser_info.user_data_file,
                id
            ],
        )?;
        if status == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
