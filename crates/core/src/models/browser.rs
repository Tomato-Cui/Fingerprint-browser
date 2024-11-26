/// 浏览器的数据结构
use crate::{config::AConfig, db::Db, errors::ApplicationServerError};
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
    pub status: bool, // 浏览器状态
}

impl Browser {
    /// browser表插入数据
    pub fn insert_browser(browser: Browser) -> Result<bool, ApplicationServerError> {
        let db = Db::new(AConfig.get_cache_location())?;

        let sql = "insert into browser_table (tags, groups, porxy, ua, ipaddr, region, os, time, fp_info, isTz, isPos, porxy_file, user_data_file, status) values(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)";
        let status = db.query_table(
            sql,
            params![
                browser.tags,
                browser.group,
                browser.porxy,
                browser.ua,
                browser.ipaddr,
                browser.region,
                browser.os,
                browser.time,
                serde_json::to_string(&browser.fp_info)?,
                browser.is_tz,
                browser.is_pos,
                browser.porxy_file,
                browser.user_data_file,
                browser.status,
            ],
        )?;
        if status == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// browser表删除数据
    pub fn delete_browser(id: i8) -> Result<bool, ApplicationServerError> {
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
    pub fn query_browser() -> Result<Vec<Browser>, ApplicationServerError> {
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
                status: row.get(14)?,
            })
        })?;

        let browser: Vec<Browser> = browser_iter.filter_map(Result::ok).collect();
        Ok(browser)
        // Ok(serde_json::to_string(&browser)?)
    }

    /// browser表查询所有数据
    /// 更新数据 由于更新的数据有好几个 为了简单索性更新全部数据 (根据id来更新数据)
    pub fn update_browser(browser: Browser, id: i8) -> Result<bool, ApplicationServerError> {
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
            user_data_file = ?13,
            status = ?14
        WHERE id = ?15
    ";
        let status = db.query_table(
            sql,
            params![
                browser.tags,
                browser.group,
                browser.porxy,
                browser.ua,
                browser.ipaddr,
                browser.region,
                browser.os,
                browser.time,
                serde_json::to_string(&browser.fp_info)?,
                browser.is_tz,
                browser.is_pos,
                browser.porxy_file,
                browser.user_data_file,
                browser.status,
                id
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
        let db = Db::new(AConfig.get_cache_location())?;

        let sql = "
            UPDATE browser_table
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

    pub fn get_browser_unique(&self) -> String {
        format!("{}.{}.{}", self.id, self.os, self.time)
    }
}
