mod enviroment;
mod fingerprint;
mod group;

#[allow(dead_code)]
pub async fn load_db() {
    use cores::database::init_database;

    crate::cores::init_config().await;
    let url = r"C:\Users\cgy\Desktop\test\test.db";
    // let url = "sqlite::memory:";
    init_database(url).await.unwrap();

    use std::path::PathBuf;
    use std::str::FromStr;
    cores::database::Database::migrator(PathBuf::from_iter(vec![
        PathBuf::from_str("..").unwrap(),
        cores::config::get_config()
            .unwrap()
            .cache
            .migrate_location
            .clone(),
    ]))
    .await
    .unwrap();
}

#[tokio::test]
async fn test_load_db() {
    load_db().await;
}
