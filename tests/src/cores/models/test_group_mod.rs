#[tokio::test]
async fn test_group_list() {
    use cores::{apis::PageParam, models::group::Group};

    crate::cores::init_config().await;

    let groups = Group::query_group(PageParam {
        page_num: Some(0),
        page_size: Some(10),
    });
    println!("{:?}", groups)
}
