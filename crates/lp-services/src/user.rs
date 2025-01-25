use lp_models::team::Team;

use crate::error::ServiceError;

pub async fn login(nickname: &str, password: &str) -> Result<String, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;
    let user = lp_models::user::User::query_email(pool, nickname).await;
    let user_info = match user {
        Ok(user) => user,
        Err(_) => return Err(ServiceError::Error("current user not exist.".to_string())),
    };

    let password = lp_commons::encryption::md5(password);
    if !user_info.password.eq(&password) {
        return Err(ServiceError::Error("password failed".to_string()));
    }
    let user = lp_models::user::User::query_user_by_user_info_id(pool, user_info.id).await?;

    let token = lp_commons::encryption::generate_token(&user.uuid)
        .map_err(|_| ServiceError::Error("generate token failed.".to_string()))?;

    lp_states::auth::set_token(&token).await;

    let default_team =
        lp_models::team::Team::query_default_team_by_user_uuid(pool, &user.uuid).await?;

    lp_models::user_use_team::UserUseTeam::create(pool, &user.uuid, default_team.id as u32).await?;
    Ok(token)
}

pub async fn regsiter(
    email: &str,
    code: &str,
    nickname: &str,
    password: &str,
) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;
    let password = lp_commons::encryption::md5(password);

    if email.is_empty() {
        return Err(ServiceError::Error("邮箱不能为空".to_string()));
    }
    let (is_ok, message) = lp_models::codes::Codes::is_ok(&pool, "register", email, code).await?;
    if !is_ok {
        return Err(ServiceError::Error(message));
    }

    let user_info = lp_models::user_info::UserInfo {
        email: email.to_string(),
        nickname: nickname.to_string(),
        password: password.to_string(),
        ..Default::default()
    };

    let uuid = lp_commons::encryption::uuid();
    let mut ok = lp_models::user::User::insert(pool, &uuid, &user_info).await?;

    if ok {
        ok = lp_models::team::Team::insert(
            pool,
            &uuid,
            &Team {
                name: email.to_string(),
                ..Default::default()
            },
            None,
        )
        .await?;
    }

    Ok(ok)
}

pub async fn query_search_by_email(email: &str) -> Result<Vec<String>, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let user_emails = lp_models::user::User::search_by_email(pool, email).await?;

    Ok(user_emails)
}

pub async fn reset_password(
    email: &str,
    password1: &str,
    password2: &str,
) -> Result<bool, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let user = lp_models::user::User::query_email(pool, email).await?;
    let password = lp_commons::encryption::md5(password1);

    if !user.password.eq(&password) {
        return Err(ServiceError::Error("password failed".to_string()));
    }

    let password = lp_commons::encryption::md5(password2);
    let ok = lp_models::user::User::update_password(pool, email, &password).await?;

    Ok(ok)
}

pub async fn logout() -> Result<bool, ServiceError> {
    lp_states::auth::clear_token().await;
    Ok(true)
}

pub async fn query_user_uuid_by_email(user_email: &str) -> Result<String, ServiceError> {
    let pool = lp_states::database::get_database_pool()?;

    let user_uuid = lp_models::user::User::query_uuid_by_email(pool, user_email).await?;

    Ok(user_uuid)
}

pub async fn register_send(email: &str) -> Result<bool, ServiceError> {
    if email.is_empty() {
        return Err(ServiceError::Error("邮箱不能为空".to_string()));
    }

    let config = if let Some(config) = lp_states::config::get_config() {
        config
    } else {
        return Err(ServiceError::Error(
            "email send failed, please resend.".to_string(),
        ));
    };

    let code = lp_commons::encryption::random_code();

    let html_body = format!(
        r#"<div style="font-family: Arial, sans-serif; color: #333; max-width: 400px; margin: 2rem auto;"><h1 style="color: #007BFF;">{}, 欢迎注册！</h1><p>这是您的验证码：</p><div style="font-size: 2rem; font-weight: bold; color: #28a745; margin: 1rem 0;">{}</div><p>如果您没有请求此验证码，请忽略此邮件。</p></div>"#,
        email, code
    );
    let ok = lp_commons::util::send_email(
        &config.email.smtp_username,
        &config.email.smtp_password,
        &config.email.smtp_server,
        email,
        &format!("{} , welcome", email),
        &html_body,
    );
    if let Err(e) = ok {
        return Err(ServiceError::Error(e.to_string()));
    }

    let pool = lp_states::database::get_database_pool()?;
    let t = lp_models::codes::Codes::insert(
        pool,
        "register",
        email,
        &code,
        &format!(
            "{}",
            (lp_commons::time::get_system_time_mills()) + 1000 * 60 * 3
        ),
    )
    .await?;

    Ok(t)
}

#[allow(unused_variables)]
pub async fn reset_password_send(
    email: &str,
    username: &str,
    password: &str,
) -> Result<String, ServiceError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::register_send;

    #[tokio::test]
    async fn test_login() {
        crate::setup().await;
        let token = super::login("lius", "lius").await;
        println!("{:?}", token);

        super::logout().await.unwrap();
    }

    #[tokio::test]
    async fn test_register() {
        crate::setup().await;
        let ok = super::regsiter("liushui_new@126.com", "810272", "1", "1").await;
        println!("{:?}", ok)
    }

    #[tokio::test]
    async fn test_reset_password() {
        crate::setup().await;
        let ok = super::reset_password("abc@abc.abc", "abc", "abc").await;
        println!("{:?}", ok)
    }

    #[tokio::test]
    async fn test_logout() {
        crate::setup().await;
        let result = super::logout().await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_reset_password_send() {
        crate::setup().await;
        let result = super::reset_password_send("abc@abc.com", "abc", "abc").await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_insert_code() {
        crate::setup().await;

        // let pool = lp_states::database::get_database_pool().unwrap();

        // let t = lp_models::codes::Codes::insert(
        //     pool,
        //     "register",
        //     "abc@abc.com",
        //     "234822",
        //     &format!(
        //         "{}",
        //         (lp_commons::time::get_system_time_mills()) + 1000 * 60 * 3
        //     ),
        // )
        // .await;
        println!("{:?}", (lp_commons::time::get_system_time_mills()),);

        // let is_ok = lp_models::codes::Codes::is_ok(pool, "register", "abc@abc.com", "234333").await;
        // println!("{:?} {:?}", t, is_ok);
    }

    #[tokio::test]
    async fn test_register_send() {
        crate::setup().await;
        let t = register_send("liushui_new@126.com").await;
        println!("{:?}", t);
    }
}
