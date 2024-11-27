use tokio::fs::{remove_dir_all, remove_file};

pub use crate::errors::ApplicationServerError;
use std::fs::metadata;

use super::common::app_localer;

/// 删除应用中的指定数据文件
pub async fn delete_data_file(base_path: &str, path: &str) -> Result<(), ApplicationServerError> {
    let path_buf = app_localer::app_data_location().join(base_path).join(path);

    if let Ok(meta) = metadata(&path_buf) {
        if meta.is_file() {
            // 如果是文件，则删除文件
            remove_file(&path_buf).await?;
        } else if meta.is_dir() {
            // 如果是目录，则删除目录及其内容
            remove_dir_all(&path_buf).await?;
        }
    }
    Ok(())
}
