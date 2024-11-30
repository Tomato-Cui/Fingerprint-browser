#[tokio::test]
async fn test_init_db_structure() {
    use super::load_db;
    use crate::cores::init_config;

    init_config().await;
    load_db().await.init_db_structure().unwrap()
}

#[tokio::test]
async fn test_query_table() {
    use super::load_db;
    use rusqlite::params;

    let sql = "insert into ua_table (os_name, os_ver, browser_ver) values(?1, ?2, ?3)";
    load_db()
        .await
        .query_table(sql, params!["abc", "abc", "abc"])
        .unwrap();
}

#[tokio::test]
async fn test_query_map_table() {
    use super::load_db;
    use cores::models::ua::Ua;
    use rusqlite::params;

    let db = load_db().await;
    let sql = "select * from ua_table";
    let mut stmt = db.query_map_table(sql).unwrap();

    let ua_iter = stmt
        .query_map(params![], |row| {
            Ok(Ua {
                id: row.get(0)?,          // 索引 0
                os_name: row.get(1)?,     // 索引 1
                os_ver: row.get(2)?,      // 索引 2
                browser_ver: row.get(3)?, // 索引 3
            })
        })
        .unwrap();

    ua_iter.for_each(|v| {
        println!("{:#?}", v);
    });
}
