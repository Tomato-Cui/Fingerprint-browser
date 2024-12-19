use std::str::FromStr;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::{Mutex, OnceCell};

pub struct State {
    pub app_cache_location: Option<PathBuf>,
}

pub static STATE: OnceCell<Arc<Mutex<State>>> = OnceCell::const_new();

pub async fn init_state() -> &'static Arc<Mutex<State>> {
    STATE
        .get_or_init(|| async {
            Arc::new(Mutex::new(State {
                app_cache_location: None,
            }))
        })
        .await
}

#[cfg(windows)]
pub async fn get_app_cache_location() -> anyhow::Result<PathBuf> {
    use anyhow::Ok;

    match STATE.get() {
        Some(s) => Ok(s
            .lock()
            .await
            .app_cache_location
            .clone()
            .ok_or(anyhow::anyhow!(
                "get app cache location faild, the variable is locked."
            )))?,
        None => {
            if let Some(app_data) = std::env::var("LOCALAPPDATA").ok() {
                Ok(PathBuf::from_str(&app_data)?)
            } else {
                Ok(PathBuf::from_str(".")?)
            }
        }
    }
}
