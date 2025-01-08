use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterPayload {
    pub email: String,
    pub code: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ResetPasswordPayload {
    pub email: String,
    pub password1: String,
    pub password2: String,
}

#[derive(Deserialize)]
pub struct RegisterSendPayload {
    pub email: String,
}

#[derive(Deserialize)]
pub struct SearchByEmailPayload {
    pub email: String,
}
