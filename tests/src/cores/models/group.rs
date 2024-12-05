#[tokio::test]
async fn test_group_insert() {
    use cores::models::group::{self};
    crate::cores::models::load_db().await;

    let result = group::Group::insert_group("abc", Some("abc".to_string())).await;

    println!("{:?}", result);
}

#[tokio::test]
async fn test_group_update() {
    use cores::models::group::{self};
    crate::cores::models::load_db().await;

    let result = group::Group::update_group("shjshfjshfsjdhfssdjffjs", Some("abc".to_string()), 1).await;

    println!("{:?}", result);
}

#[tokio::test]
async fn test_group_delete() {
    use cores::models::group::{self};
    crate::cores::models::load_db().await;

    let result = group::Group::delete_group(1).await.unwrap();

    println!("{:?}", result);
}

#[tokio::test]
async fn test_group_query() {
    use cores::models::group::{self};
    crate::cores::models::load_db().await;

    let result = group::Group::query_group(&cores::apis::PageParam {
        page_num: Some(1),
        page_size: Some(10),
    })
    .await;

    println!("{:?}", result);
}
