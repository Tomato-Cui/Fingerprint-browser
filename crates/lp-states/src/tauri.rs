use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};

pub struct State {
    // pub tx: Sender<String>,
    // pub rx: Sender<String>,
}

pub static STATE: OnceCell<Arc<RwLock<State>>> = OnceCell::const_new();

pub async fn init_state() -> &'static Arc<RwLock<State>> {
    STATE
        .get_or_init(|| async {
            Arc::new(RwLock::new(State {
                // app_cache_location: None,
            }))
        })
        .await
}

// pub async fn get_app_cache_location() -> PathBuf {
//     if let Some(s) = STATE.get() {
//         if let Some(location) = s.read().await.app_cache_location.clone() {
//             return location;
//         }
//     }
//     std::env::current_dir().unwrap()
// }

// pub async fn set_app_cache_location(path_buf: PathBuf) {
//     if let Some(state) = STATE.get() {
//         let mut state = state.write().await;
//         state.app_cache_location = Some(path_buf);
//     }
// }
