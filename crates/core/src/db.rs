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
            CREATE TABLE IF NOT EXISTS browser_table (
                    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                    tags TEXT NOT NULL,
                    groups TEXT NOT NULL,
                    porxy TEXT NOT NULL,
                    ua TEXT NOT NULL,
                    ipaddr TEXT NOT NULL,
                    region TEXT NOT NULL,
                    os TEXT NOT NULL,
                    time INTEGER,
                    fp_info TEXT NOT NULL,
                    isTz INTEGER,
                    isPos INTEGER,
                    porxy_file TEXT NOT NULL,
                    user_data_file TEXT NOT NULL,
                    status  INTEGER
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
