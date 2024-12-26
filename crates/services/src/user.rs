use crate::error::ServiceError;

pub async fn login(nickname: &str, password: &str) -> Result<String, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let user = models::user::User::query_nickname(pool, nickname).await;
    let user_info = match user {
        Ok(user) => user,
        Err(_) => return Err(ServiceError::Error("current user not exist.".to_string())),
    };

    let password = commons::encryption::md5(password);
    if !user_info.password.eq(&password) {
        return Err(ServiceError::Error("password failed".to_string()));
    }
    let user = models::user::User::query_user_by_user_info_id(pool, user_info.id).await?;

    let token = commons::encryption::generate_token(&user.uuid)
        .map_err(|_| ServiceError::Error("generate token failed.".to_string()))?;

    states::auth::set_token(&token).await;

    Ok(token)
}

pub async fn regsiter(email: &str, nickname: &str, password: &str) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let password = commons::encryption::md5(password);

    let user_info = models::user_info::UserInfo {
        email: email.to_string(),
        nickname: nickname.to_string(),
        password: password.to_string(),
        ..Default::default()
    };

    let uuid = commons::encryption::uuid();
    let ok = models::user::User::insert(pool, &uuid, &user_info).await?;

    Ok(ok)
}

pub async fn reset_password(
    email: &str,
    password1: &str,
    password2: &str,
) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let user = models::user::User::query_email(pool, email).await?;
    let password = commons::encryption::md5(password1);

    if !user.password.eq(&password) {
        return Err(ServiceError::Error("password failed".to_string()));
    }

    let password = commons::encryption::md5(password2);
    let ok = models::user::User::update_password(pool, email, &password).await?;

    Ok(ok)
}

pub async fn logout() -> Result<bool, ServiceError> {
    states::auth::clear_token().await;
    Ok(true)
}

#[allow(unused_variables)]
pub async fn register_send(
    email: &str,
    username: &str,
    password: &str,
) -> Result<String, ServiceError> {
    todo!()
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
    #[tokio::test]
    async fn test_login() {
        crate::setup().await;
        let token = super::login("abc", "abc").await;
        println!("{:?}", token);

        super::logout().await.unwrap();
    }

    #[tokio::test]
    async fn test_register() {
        crate::setup().await;
        let ok = super::regsiter("abc@abc.com", "abc", "abc").await;
        println!("{:?}", ok)
    }

    #[tokio::test]
    async fn test_reset_password() {
        crate::setup().await;
        let ok = super::reset_password("abc@abc.com", "123", "123").await;
        println!("{:?}", ok)
    }
}
