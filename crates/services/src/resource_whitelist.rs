use models::resource_whitelist::ResourceWhiteList;

use crate::error::ServiceError;

pub async fn exists(href: &str, method: &str) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let _ = ResourceWhiteList::query_by_href_method(pool, href, method).await?;

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_exists_success() {
        let href = "";
        let method = "";
        let result = exists(href, method).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }
}
