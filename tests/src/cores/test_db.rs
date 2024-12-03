#[tokio::test]
async fn test_migrate() {
    use crate::cores::init_config;
    use cores::database::Database;

    use cores::config::get_config;

    init_config().await;
    crate::cores::models::load_db().await;

    Database::migrator(get_config().unwrap().cache.migrate_location.clone())
        .await
        .unwrap();
}
