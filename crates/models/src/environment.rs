use serde::{Deserialize, Serialize};
use sqlx::{error::Error, FromRow, Pool, Sqlite};

use crate::user;

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, Default)]
pub struct Environment {
    #[serde(skip)]
    pub owner_id: Option<i32>, // 所有者ID
    pub id: Option<i32>,                   // 自增ID
    pub name: String,                      // 环境名称
    pub description: Option<String>,       // 环境名称
    pub domain_name: Option<String>,       // 账号平台的域名
    pub open_urls: Option<String>,         // 其他URL
    pub repeat_config: Option<String>,     // 去重配置
    pub username: String,                  // 账号
    pub password: String,                  // 密码
    pub fakey: Option<String>,             // 2FA密钥
    pub cookie: Option<String>,            // Cookie
    pub ignore_cookie_error: Option<i8>,   // 校验Cookie失败时的行为
    pub group_id: Option<i32>,             // 分组ID
    pub fp_info_id: Option<i32>,           // 指纹信息ID
    pub ua: String,                        // 用户代理
    pub os: String,                        // 操作系统
    pub country: Option<String>,           // 国家/地区
    pub region: Option<String>,            // 省/州
    pub city: Option<String>,              // 城市
    pub remark: Option<String>,            // 备注
    pub ipchecker: String,                 // IP查询渠道
    pub sys_app_cate_id: String,           // 应用分类ID
    pub user_proxy_config: Option<String>, // 环境代理配置
    pub proxy_enable: i8,                  // 代理启用
    pub is_tz: i8,                         // 是否启用时区
    pub is_pos: i8,                        // 是否启用地理位置
    pub user_data_file: String,            // 用户数据文件路径
    pub driver_location: Option<String>,   // 浏览器驱动位置
    pub status: i8,                        // 浏览器状态
    pub created_at: Option<String>,        // 创建时间
    pub updated_at: Option<String>,        // 更新时间
    pub lasted_at: Option<String>,         // 最近时间
    pub deleted_at: Option<String>,        // 删除时间
    pub deleted_id: Option<u32>,        // 删除时间
    pub deleted_username: Option<String>,        // 删除时间
}

impl Environment {
    #[allow(dead_code)]
    pub async fn insert(
        pool: &Pool<Sqlite>,
        user_id: u32,
        environment: &Environment,
    ) -> Result<bool, Error> {
        let sql = r#"
        INSERT INTO environments (
            name, owner_id, description,domain_name, open_urls, repeat_config, username, password, 
            fakey, cookie, ignore_cookie_error, group_id, fp_info_id, ua, os, country, region, city, remark, 
            ipchecker, sys_app_cate_id, user_proxy_config, proxy_enable, is_tz, is_pos, user_data_file, driver_location, status
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#;

        let row = sqlx::query(sql)
            .bind(&environment.name)
            .bind(user_id)
            .bind(&environment.description)
            .bind(&environment.domain_name)
            .bind(&environment.open_urls)
            .bind(&environment.repeat_config)
            .bind(&environment.username)
            .bind(&environment.password)
            .bind(&environment.fakey)
            .bind(&environment.cookie)
            .bind(environment.ignore_cookie_error)
            .bind(environment.group_id)
            .bind(environment.fp_info_id)
            .bind(&environment.ua)
            .bind(&environment.os)
            .bind(&environment.country)
            .bind(&environment.region)
            .bind(&environment.city)
            .bind(&environment.remark)
            .bind(&environment.ipchecker)
            .bind(&environment.sys_app_cate_id)
            .bind(&environment.user_proxy_config)
            .bind(environment.proxy_enable)
            .bind(environment.is_tz)
            .bind(environment.is_pos)
            .bind(&environment.user_data_file)
            .bind(&environment.driver_location)
            .bind(&environment.status)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn query_by_id(
        pool: &Pool<Sqlite>,
        user_id: Option<u32>,
        id: u32,
        group_id: Option<u32>,
    ) -> Result<Environment, Error> {
        let mut query_builder = sqlx::query_as(
            "select * from environments where id = ? and (owner_id = ? or group_id = ?)",
        )
        .bind(id);
        if let Some(owner_id) = user_id {
            query_builder = query_builder.bind(owner_id);
        } else {
            query_builder = query_builder.bind(None::<i32>);
        };

        if let Some(group_id) = group_id {
            query_builder = query_builder.bind(group_id);
        } else {
            query_builder = query_builder.bind(None::<i32>);
        };

        let enviroment: Environment = query_builder.fetch_one(pool).await?;

        Ok(enviroment)
    }
    #[allow(dead_code)]
    pub async fn query_delete(
        pool: &Pool<Sqlite>,
        user_id: u32,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<Environment>), Error> {
        let current_user = user::User::query_id(pool, user_id as i32).await?;

        let (total,): (i64,) = sqlx::query_as(
            "select count(1) from environments where owner_id = ? and deleted_at is not NULL",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        let mut environments: Vec<Environment> =
            sqlx::query_as("select * from environments where owner_id = ? and deleted_at is not NULL limit ? offset ?")
                .bind(user_id)
                .bind(page_size)
                .bind(offset)
                .fetch_all(pool)
                .await?;
        environments
            .iter_mut()
            .for_each(|v| v.deleted_username= Some(current_user.nickname.clone()));

        Ok((total, environments))
    }

    #[allow(dead_code)]
    pub async fn query_by_col(
        pool: &Pool<Sqlite>,
        user_id: u32,
        col_name: &str,
        col_value: &str,
        page_num: u32,
        page_size: u32,
    ) -> Result<(i64, Vec<Environment>), Error> {
        let (total,): (i64,) = if col_name.is_empty() {
            sqlx::query_as("select count(1) from environments where owner_id = ? and deleted_at is not null")
                .bind(user_id)
                .fetch_one(pool)
                .await?
        } else {
            sqlx::query_as(&format!(
                "select count(1) from environments where {} = ? and owner_id = ? and deleted_at is not null",
                col_name
            ))
            .bind(col_value)
            .bind(user_id)
            .fetch_one(pool)
            .await?
        };

        let page_num = if page_num <= 0 || ((page_num * page_size) as i64) > total {
            0
        } else {
            page_num
        };
        let offset = page_num * page_size;

        let environments: Vec<Environment> = if col_name.is_empty() {
            sqlx::query_as("select * from environments where owner_id = ? and deleted_at is not null limit ? offset ? ")
                .bind(user_id)
                .bind(page_size)
                .bind(offset)
                .fetch_all(pool)
                .await?
        } else {
            sqlx::query_as(&format!(
                "select * from environments where {} = ? and owner_id = ? and deleted_at is not null  limit ? offset ?",
                col_name
            ))
            .bind(col_value)
            .bind(user_id)
            .bind(page_size)
            .bind(offset)
            .fetch_all(pool)
            .await?
        };

        Ok((total, environments))
    }

    #[allow(dead_code)]
    pub async fn update(
        pool: &Pool<Sqlite>,
        user_id: u32,
        environment: &Environment,
    ) -> Result<bool, Error> {
        let sql = "
            UPDATE environments
            SET name = ?, description = ?, domain_name = ?, open_urls = ?, repeat_config = ?, username = ?, password = ?, fakey = ?, cookie = ?,
                ignore_cookie_error = ?, group_id = ?, fp_info_id = ?, ua = ?, os = ?, country = ?, region = ?, city = ?, remark = ?, ipchecker = ?,
                sys_app_cate_id = ?, user_proxy_config = ?, proxy_enable = ?, is_tz = ?, is_pos = ?, user_data_file = ?, driver_location = ?, status = ?, 
                updated_at = DATETIME('now') 
            WHERE id = ? and owner_id = ?
        ";

        let row = sqlx::query(sql)
            .bind(&environment.name)
            .bind(&environment.description)
            .bind(&environment.domain_name)
            .bind(&environment.open_urls)
            .bind(&environment.repeat_config)
            .bind(&environment.username)
            .bind(&environment.password)
            .bind(&environment.fakey)
            .bind(&environment.cookie)
            .bind(environment.ignore_cookie_error)
            .bind(environment.group_id)
            .bind(environment.fp_info_id)
            .bind(&environment.ua)
            .bind(&environment.os)
            .bind(&environment.country)
            .bind(&environment.region)
            .bind(&environment.city)
            .bind(&environment.remark)
            .bind(&environment.ipchecker)
            .bind(&environment.sys_app_cate_id)
            .bind(&environment.user_proxy_config)
            .bind(environment.proxy_enable as i32)
            .bind(environment.is_tz as i32)
            .bind(environment.is_pos as i32)
            .bind(format!(
                "{}.{}",
                &environment.user_data_file,
                commons::time::get_system_time_mills()
            ))
            .bind(&environment.driver_location)
            .bind(environment.status)
            .bind(&environment.id)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn update_by_col(
        pool: &Pool<Sqlite>,
        user_id: u32,
        id: u32,
        col_name: &str,
        col_value: &str,
    ) -> Result<bool, Error> {
        if col_name.is_empty() {
            return Err(sqlx::error::Error::ColumnNotFound(format!(
                "{} column not found",
                col_name
            )));
        }
        let row = sqlx::query(&format!(
            "UPDATE environments SET {} = ?,updated_at = DATETIME('now') WHERE id = ? and owner_id = ?",
            col_name
        ))
        .bind(col_value)
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn delete(pool: &Pool<Sqlite>, user_id: u32, id: u32) -> Result<bool, Error> {
        let row = sqlx::query(
            "UPDATE environments SET deleted_at = DATETIME('now'), deleted_id = ? WHERE id = ? and owner_id = ?",
        )
        .bind(user_id)
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn delete_again(pool: &Pool<Sqlite>, user_id: u32, id: u32) -> Result<bool, Error> {
        let row = sqlx::query(
            " DELETE FROM environments WHERE id = ? and owner_id = ? and deleted_at is not null",
        )
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn recover(pool: &Pool<Sqlite>, user_id: u32, id: u32) -> Result<bool, Error> {
        let row =
            sqlx::query("UPDATE environments SET deleted_at = NULL WHERE id = ? and owner_id = ?")
                .bind(id)
                .bind(user_id)
                .execute(pool)
                .await?;
        Ok(row.rows_affected() == 1)
    }

    #[allow(dead_code)]
    pub async fn recover_all(pool: &Pool<Sqlite>, user_id: u32) -> Result<bool, Error> {
        let row = sqlx::query("UPDATE environments SET deleted_at = NULL WHERE owner_id = ?")
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(row.rows_affected() > 1)
    }

    #[allow(dead_code)]
    pub async fn clean(pool: &Pool<Sqlite>, user_id: u32) -> Result<bool, Error> {
        let row =
            sqlx::query(" DELETE FROM environments WHERE owner_id = ? and deleted_at is not null")
                .bind(user_id)
                .execute(pool)
                .await?;
        Ok(row.rows_affected() == 1)
    }
}
