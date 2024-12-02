use tokio::sync::{OnceCell, RwLock};

pub struct AuthState {
    pub token: Option<String>,
}

pub static AUTH_STATE: OnceCell<RwLock<AuthState>> = OnceCell::const_new();

pub async fn init_auth_state() -> &'static RwLock<AuthState> {
    AUTH_STATE
        .get_or_init(|| async { RwLock::new(AuthState { token: None }) })
        .await
}

pub async fn get_token() -> Option<String> {
    if let Some(state) = AUTH_STATE.get() {
        state.read().await.token.clone()
    } else {
        None
    }
}
pub async fn clear_token() {
    if let Some(state) = AUTH_STATE.get() {
        let mut auth_state = state.write().await;
        auth_state.token = None
    }
}

pub async fn set_token(token: &str) {
    if let Some(state) = AUTH_STATE.get() {
        let mut auth_state = state.write().await;
        auth_state.token = Some(token.to_string());
    }
}
