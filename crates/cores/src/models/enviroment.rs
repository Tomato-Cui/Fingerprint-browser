/// 浏览器的数据结构
use crate::{
    apis::PageParam, database::get_db, errors::ApplicationServerError,
    utils::common::app_timer::generate_nanosecond_timestamp,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

//定义浏览器环境数据结构
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Environment {
    pub id: Option<i8>,                    // 自增ID
    pub name: String,                      // 环境名称
    pub domain_name: String,               // 账号平台的域名
    pub open_urls: Option<String>,         // 其他URL
    pub repeat_config: Option<String>,     // 去重配置
    pub username: String,                  // 账号
    pub password: String,                  // 密码
    pub fakey: String,                     // 2FA密钥
    pub cookie: Option<String>,            // Cookie
    pub ignore_cookie_error: Option<i8>,   // 校验Cookie失败时的行为
    pub group_id: Option<i8>,              // 分组ID
    pub fp_info_id: Option<i8>,            // 指纹信息ID
    pub ua: String,                        // 用户代理
    pub os: String,                        // 操作系统
    pub country: Option<String>,           // 国家/地区
    pub region: Option<String>,            // 省/州
    pub city: Option<String>,              // 城市
    pub remark: Option<String>,            // 备注
    pub ipchecker: String,                 // IP查询渠道
    pub sys_app_cate_id: String,           // 应用分类ID
    pub user_proxy_config: Option<String>, // 环境代理配置
    pub proxy: Option<String>,             // 代理IP
    pub proxy_enable: bool,                // 代理启用
    pub is_tz: bool,                       // 是否启用时区
    pub is_pos: bool,                      // 是否启用地理位置
    pub user_data_file: String,            // 用户数据文件路径
    pub status: bool,                      // 浏览器状态
    pub created_at: Option<String>,        // 创建时间
    pub updated_at: Option<String>,        // 更新时间
    pub deleted_at: Option<String>,        // 删除时间
}

impl Environment {
    ///Environment 表插入数据
    pub async fn insert(environment: Environment) -> Result<bool, ApplicationServerError> {
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
        group_id, 
        fp_info_id, 
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
        is_tz, 
        is_pos, 
        user_data_file, 
        status
    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26)";

        let row = sqlx::query(sql)
            .bind(&environment.name) // name
            .bind(&environment.domain_name) // domain_name
            .bind(&environment.open_urls.unwrap_or_default()) // open_urls
            .bind(&environment.repeat_config.unwrap_or_default()) // repeat_config
            .bind(&environment.username) // username
            .bind(&environment.password) // password
            .bind(&environment.fakey) // fakey
            .bind(&environment.cookie.unwrap_or_default()) // cookie
            .bind(environment.ignore_cookie_error.unwrap_or(0)) // ignore_cookie_error
            .bind(environment.group_id.unwrap_or(0)) // group_id
            .bind(environment.fp_info_id) // fp_info_id
            .bind(&environment.ua) // ua
            .bind(&environment.os) // os
            .bind(environment.country.unwrap_or_default()) // country
            .bind(environment.region.unwrap_or_default()) // region
            .bind(environment.city.unwrap_or_default()) // city
            .bind(environment.remark.unwrap_or_default()) // remark
            .bind(&environment.ipchecker) // ipchecker
            .bind(&environment.sys_app_cate_id) // sys_app_cate_id
            .bind(environment.user_proxy_config.unwrap_or_default()) // user_proxy_config
            .bind(environment.proxy.unwrap_or_default()) // proxy
            .bind(environment.proxy_enable as i32) // proxy_enable (bool -> i32)
            .bind(environment.is_tz as i32) // is_tz (bool -> i32)
            .bind(environment.is_pos as i32) // is_pos (bool -> i32)
            .bind(&environment.user_data_file) // user_data_file
            .bind(environment.status as i32) // status (bool -> i32)
            .bind(environment.created_at.unwrap_or_default()) // created_at
            .bind(environment.updated_at.unwrap_or_default()) // updated_at
            .bind(environment.deleted_at.unwrap_or_default()) // deleted_at
            .execute(get_db()?) // execute the query
            .await?;

        Ok(row.rows_affected() == 1)
    }

    /// envirionment表删除数据
    pub async fn delete_envirionment(ids: Vec<u8>) -> Result<bool, ApplicationServerError> {
        let delete_ids = ids
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<String>>()
            .join(",");

        let sql = format!("delete from environments where id in ({})", delete_ids);
        let row = sqlx::query(&sql).execute(get_db()?).await?;
        Ok(row.rows_affected() == 1)
    }

    /// envirionment表查询指定id数据
    pub async fn query_envirionment_by_id(id: i8) -> Result<Environment, ApplicationServerError> {
        let pool = get_db()?;
        let enviroment: Environment = sqlx::query_as("select * from environments where id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(enviroment)
    }

    /// envirionment表查询所有数据
    pub async fn query_envirionment(
        payload: PageParam,
    ) -> Result<Vec<Environment>, ApplicationServerError> {
        let page_num = payload.page_num.unwrap_or_else(|| 0);
        let page_size = payload.page_size.unwrap_or_else(|| 10);
        let offset = page_num * page_size;

        let environments: Vec<Environment> =
            sqlx::query_as("select * from environments limit $1 offset $2")
                .bind(page_size)
                .bind(offset)
                .fetch_all(get_db()?)
                .await?;

        Ok(environments)
    }

    /// envirionment表查询所有数据
    /// 更新数据 由于更新的数据有好几个 为了简单索性更新全部数据 (根据id来更新数据)
    pub async fn update_environment(
        environment: Environment,
    ) -> Result<bool, ApplicationServerError> {
        let sql = "
    UPDATE environments
    SET 
        name = ?1,
        domain_name = ?2,
        open_urls = ?3,
        repeat_config = ?4,
        username = ?5,
        password = ?6,
        fakey = ?7,
        cookie = ?8,
        ignore_cookie_error = ?9,
        group_id = ?11,
        fp_info_id = ?12,
        ua = ?13,
        os = ?14,
        country = ?15,
        region = ?16,
        city = ?17,
        remark = ?18,
        ipchecker = ?19,
        sys_app_cate_id = ?20,
        user_proxy_config = ?21,
        proxy = ?22,
        proxy_enable = ?23,
        is_tz = ?24,
        is_pos = ?25,
        user_data_file = ?26,
        status = ?27,
        updated_at = now()
    WHERE id = ?28
";

        let row = sqlx::query(sql)
            .bind(&environment.name) // name
            .bind(&environment.domain_name) // domain_name
            .bind(environment.open_urls.unwrap_or_default()) // open_urls
            .bind(environment.repeat_config.unwrap_or_default()) // repeat_config
            .bind(&environment.username) // username
            .bind(&environment.password) // password
            .bind(&environment.fakey) // fakey
            .bind(environment.cookie.unwrap_or_default()) // cookie
            .bind(environment.ignore_cookie_error.unwrap_or(0)) // ignore_cookie_error
            .bind(environment.group_id.unwrap_or(0)) // group_id
            .bind(environment.fp_info_id) // fp_info_id
            .bind(&environment.ua) // ua
            .bind(&environment.os) // os
            .bind(environment.country.unwrap_or_default()) // country
            .bind(environment.region.unwrap_or_default()) // region
            .bind(environment.city.unwrap_or_default()) // city
            .bind(environment.remark.unwrap_or_default()) // remark
            .bind(&environment.ipchecker) // ipchecker
            .bind(&environment.sys_app_cate_id) // sys_app_cate_id
            .bind(environment.user_proxy_config.unwrap_or_default()) // user_proxy_config
            .bind(environment.proxy.unwrap_or_default()) // proxy
            .bind(environment.proxy_enable as i32) // proxy_enable (bool -> i32)
            .bind(environment.is_tz as i32) // is_tz (bool -> i32)
            .bind(environment.is_pos as i32) // is_pos (bool -> i32)
            .bind(format!(
                "{}.{}",
                &environment.user_data_file,
                generate_nanosecond_timestamp() // Ensure timestamp is added
            )) // user_data_file
            .bind(environment.status as i32) // status (bool -> i32)
            .bind(environment.updated_at) //
            .bind(&environment.id) // id
            .execute(get_db()?) // Execute the query
            .await?;

        Ok(row.rows_affected() == 1)
    }

    /// envirionment表更新浏览器状态
    ///
    /// 可以通过这个方法来更新浏览器的启动和关闭
    pub async fn update_envirionment_status(
        id: i8,
        status: bool,
    ) -> Result<bool, ApplicationServerError> {
        let row = sqlx::query("UPDATE environments SET status = ?1 WHERE id = ?2")
            .bind(status)
            .bind(id)
            .execute(get_db()?)
            .await?;
        Ok(row.rows_affected() == 1)
    }

    /// envirionment表更新浏览器代理
    pub async fn update_envirionment_proxy(
        id: i8,
        proxy: Option<&str>,
    ) -> Result<bool, ApplicationServerError> {
        let row = sqlx::query("UPDATE environments SET proxy = ?1 WHERE id = ?2")
            .bind(proxy)
            .bind(id)
            .execute(get_db()?)
            .await?;
        Ok(row.rows_affected() == 1)
    }

    /// envirionment表更新浏览器ua
    ///
    /// empty ua 已经在函数外被筛选
    pub async fn update_envirionment_ua(id: i8, ua: &str) -> Result<bool, ApplicationServerError> {
        let row = sqlx::query("UPDATE environments SET ua = ?1 WHERE id = ?2")
            .bind(ua)
            .bind(id)
            .execute(get_db()?)
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn update_envirionment_group(
        id: u8,
        group_id: u8,
    ) -> Result<bool, ApplicationServerError> {
        let row = sqlx::query("UPDATE environments SET group_id = ?1 WHERE id = ?2")
            .bind(group_id)
            .bind(id)
            .execute(get_db()?)
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub fn get_envirionment_unique(&self) -> String {
        format!(
            "{}.{}.{}",
            self.id.unwrap_or_default(),
            self.is_pos,
            self.os,
        )
    }
}
