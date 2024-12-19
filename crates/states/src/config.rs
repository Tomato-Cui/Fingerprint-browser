use commons::config::AppConfig;
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
