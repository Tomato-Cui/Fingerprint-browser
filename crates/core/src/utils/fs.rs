pub use crate::errors::ApplicationServerError;
use std::fs::{metadata, remove_dir_all, remove_file};

use super::common::app_localer;

/// 删除应用中的指定数据文件
pub fn delete_data_file(base_path: &str, path: &str) -> Result<(), ApplicationServerError> {
    let path_buf = app_localer::app_data_location().join(base_path).join(path);
    println!("{:?}", path_buf);

    if let Ok(meta) = metadata(&path_buf) {
        if meta.is_file() {
            // 如果是文件，则删除文件
            remove_file(&path_buf)?;
        } else if meta.is_dir() {
            // 如果是目录，则删除目录及其内容
            remove_dir_all(&path_buf)?;
        }
    }
    Ok(())
}
