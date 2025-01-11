use tokio::sync::{OnceCell, RwLock};

#[allow(dead_code)]
pub(crate) struct State {
    pub(crate) token: Option<String>,
}

static STATE: OnceCell<RwLock<State>> = OnceCell::const_new();

#[allow(dead_code)]
pub(crate) async fn init_state() -> &'static RwLock<State> {
    STATE
        .get_or_init(|| async { RwLock::new(State { token: None }) })
        .await
}

pub async fn get_token() -> Option<String> {
    if let Some(state) = STATE.get() {
        state.read().await.token.clone()
    } else {
        None
    }
}

pub async fn clear_token() {
    if let Some(state) = STATE.get() {
        let mut auth_state = state.write().await;
        auth_state.token = None
    }
}

pub async fn set_token(token: &str) {
    if let Some(state) = STATE.get() {
        let mut auth_state = state.write().await;
        auth_state.token = Some(token.to_string());
    }
}
