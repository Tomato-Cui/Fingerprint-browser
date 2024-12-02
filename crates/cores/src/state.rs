use std::{path::PathBuf, sync::Arc};

use tokio::sync::{Mutex, OnceCell};

pub struct State {
    pub app_cache_location: PathBuf,
}

pub static STATE: OnceCell<Arc<Mutex<State>>> = OnceCell::const_new();

pub async fn init_state(state: State) -> &'static Arc<Mutex<State>> {
    STATE
        .get_or_init(|| async { Arc::new(Mutex::new(state)) })
        .await
}

pub async fn get_app_cache_location() -> crate::Result<PathBuf> {
    match STATE.get() {
        Some(s) => Ok(s.lock().await.app_cache_location.clone()),
        None => Err(crate::errors::ApplicationServerError::Error(
            anyhow::anyhow!("get app cache locaton fail"),
        )),
    }
}
