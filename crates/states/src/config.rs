use commons::config::AppConfig;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use tokio::sync::OnceCell;

pub struct State {
    pub app_config: AppConfig,
}

pub static STATE: OnceCell<State> = OnceCell::const_new();

pub async fn init_state_str(config: &str) -> &'static State {
    STATE
        .get_or_init(|| async {
            let app_config = AppConfig::build_by_str(config).unwrap();
            State { app_config }
        })
        .await
}

pub async fn init_state_path(path: &str) -> &'static State {
    STATE
        .get_or_init(|| async {
            let app_config = AppConfig::build_by_filepath(path).unwrap();
            State { app_config }
        })
        .await
}

pub fn get_config() -> Option<&'static AppConfig> {
    STATE.get().map(|s| &s.app_config)
}

pub static APP_DATA: Lazy<PathBuf> = Lazy::new(|| {
    let config = STATE.get().map(|s| &s.app_config).unwrap();
    if let Some(path_buf) = dirs::data_local_dir() {
        path_buf.join(PathBuf::from_iter(vec![format!("{}", config.app.id)]))
    } else {
        std::env::current_dir()
            .unwrap()
            .join(PathBuf::from_iter(vec![format!("{}", config.app.id)]))
    }
});
