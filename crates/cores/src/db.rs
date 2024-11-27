use rusqlite::{Connection, Params, Statement};
use std::{fs::metadata, path::PathBuf};

use crate::errors::ApplicationServerError;

// db manager
pub struct Db {
    pub conn: Connection,
    pub db_file_path: PathBuf,
}

impl Db {
    /// new db struct client
    pub fn new(db_file_path: PathBuf) -> Result<Self, ApplicationServerError> {
        Ok(Db {
            conn: Connection::open(&db_file_path)?,
            db_file_path,
        })
    }

    /// init table structure
    pub fn init_db_structure(&self) -> Result<(), ApplicationServerError> {
        // 先判断文件长度
        let meta = metadata(&self.db_file_path)?;
        let sqls = vec![
            "CREATE TABLE IF NOT EXISTS ua_table (
                id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                os_name TEXT NOT NULL,
                os_ver TEXT NOT NULL,
                browser_ver TEXT NOT NULL
            )",
            "
            CREATE TABLE IF NOT EXISTS gpu_table (
                id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                gpu_ven TEXT NOT NULL,
                gpu_rend TEXT NOT NULL,
                os_name TEXT NOT NULL
            )",
            "
            CREATE TABLE IF NOT EXISTS environments (
                id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                name TEXT,
                domain_name TEXT,
                open_urls TEXT,
                repeat_config INTEGER,
                username TEXT,
                password TEXT,
                fakey TEXT,
                cookie TEXT,
                ignore_cookie_error INTEGER,
                tags TEXT,
                group_id TEXT,
                ua TEXT NOT NULL,
                os TEXT NOT NULL,
                country TEXT,
                region TEXT,
                city TEXT,
                remark TEXT,
                ipchecker TEXT,
                sys_app_cate_id INTEGER,
                user_proxy_config TEXT,
                proxy TEXT,
                proxy_enable INTEGER NOT NULL,
                fingerprint_config TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                fp_info TEXT NOT NULL,
                is_tz INTEGER NOT NULL,
                is_pos INTEGER NOT NULL,
                user_data_file TEXT NOT NULL,
                status INTEGER NOT NULL
            )",
        ];
        if meta.len() == 0 {
            for sql in sqls {
                self.conn.execute(&sql, [])?;
            }
        }
        Ok(())
    }

    /// query table
    pub fn query_table<P: Params>(
        &self,
        sql: &str,
        params: P,
    ) -> Result<usize, ApplicationServerError> {
        Ok(self.conn.execute(sql, params)?)
    }

    /// query table to map
    pub fn query_map_table(&self, sql: &str) -> Result<Statement, ApplicationServerError> {
        let stmt: rusqlite::Statement = self.conn.prepare(sql)?;
        Ok(stmt)
    }
}
