use models::environment_proxies::Proxy;
use serde_json::{json, Value};

use crate::error::ServiceError;

pub async fn query_by_id(user_uuid: &str, id: u32) -> Result<Proxy, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let proxy = Proxy::query_proxy_by_id(pool, id, user_uuid).await?;

    Ok(proxy)
}

pub async fn query_by_uuid(
    user_uuid: &str,
    page_num: u32,
    page_size: u32,
) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;
    let (total, proxies) =
        Proxy::query_proxy_groups_by_user_uuid(pool, user_uuid, page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": proxies,
    }))
}

pub async fn query(user_uuid: &str, page_num: u32, page_size: u32) -> Result<Value, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let (total, proxies) =
        Proxy::query_proxies_by_user_uuid(pool, user_uuid, page_num, page_size).await?;

    Ok(json!({
        "total": total,
        "data": proxies,
    }))
}

pub async fn create(user_uuid: &str, mut payload: Proxy) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    payload.user_uuid = user_uuid.to_string();

    let ok = Proxy::insert_proxy(pool, &payload).await?;

    Ok(ok)
}

pub async fn update(user_uuid: &str, id: u32, payload: Proxy) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Proxy::update_proxy(pool, id, user_uuid, &payload).await?;

    Ok(ok)
}

pub async fn move_to_proxy_group(proxy_id: u32, proxy_group_id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Proxy::update_proxy_group_for_proxy(pool, proxy_id, proxy_group_id).await?;

    Ok(ok)
}

pub async fn delete(user_uuid: &str, id: u32) -> Result<bool, ServiceError> {
    let pool = states::database::get_database_pool()?;

    let ok = Proxy::delete_proxy(pool, id, user_uuid).await?;

    Ok(ok)
}
// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn setup() -> Runtime {
//         Runtime::new().unwrap()
//     }

//     #[test]
//     fn test_query_by_id() {
//         let rt = setup();
//         rt.block_on(async {
//             let user_uuid = "test_user_uuid";
//             let id = 1;
//             let result = query_by_id(user_uuid, id).await;
//             assert!(result.is_ok());
//         });
//     }

//     #[test]
//     fn test_query_by_uuid() {
//         let rt = setup();
//         rt.block_on(async {
//             let user_uuid = "test_user_uuid";
//             let page_num = 1;
//             let page_size = 10;
//             let result = query_by_uuid(user_uuid, page_num, page_size).await;
//             assert!(result.is_ok());
//             let value: Value = result.unwrap();
//             assert!(value["total"].is_number());
//             assert!(value["data"].is_array());
//         });
//     }

//     #[test]
//     fn test_query() {
//         let rt = setup();
//         rt.block_on(async {
//             let user_uuid = "test_user_uuid";
//             let page_num = 1;
//             let page_size = 10;
//             let result = query(user_uuid, page_num, page_size).await;
//             assert!(result.is_ok());
//             let value: Value = result.unwrap();
//             assert!(value["total"].is_number());
//             assert!(value["data"].is_array());
//         });
//     }

//     #[test]
//     fn test_create() {
//         let rt = setup();
//         rt.block_on(async {
//             let user_uuid = "test_user_uuid";
//             let payload = Proxy {
//                 id: 0,
//                 user_uuid: user_uuid.to_string(),
//                 // other fields initialization
//             };
//             let result = create(user_uuid, payload).await;
//             assert!(result.is_ok());
//             assert!(result.unwrap());
//         });
//     }

//     #[test]
//     fn test_update() {
//         let rt = setup();
//         rt.block_on(async {
//             let user_uuid = "test_user_uuid";
//             let id = 1;
//             let payload = Proxy {
//                 id,
//                 user_uuid: user_uuid.to_string(),
//                 // other fields initialization
//             };
//             let result = update(user_uuid, id, payload).await;
//             assert!(result.is_ok());
//             assert!(result.unwrap());
//         });
//     }

//     #[test]
//     fn test_move_to_proxy_group() {
//         let rt = setup();
//         rt.block_on(async {
//             let proxy_id = 1;
//             let proxy_group_id = 2;
//             let result = move_to_proxy_group(proxy_id, proxy_group_id).await;
//             assert!(result.is_ok());
//             assert!(result.unwrap());
//         });
//     }

//     #[test]
//     fn test_delete() {
//         let rt = setup();
//         rt.block_on(async {
//             let user_uuid = "test_user_uuid";
//             let id = 1;
//             let result = delete(user_uuid, id).await;
//             assert!(result.is_ok());
//             assert!(result.unwrap());
//         });
//     }
// }
