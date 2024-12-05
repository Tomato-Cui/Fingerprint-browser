#[tokio::test]
async fn test_fingerprint_insert() {
    use cores::models::fingerprint::{self};
    crate::cores::models::load_db().await;

    let fingerprint_info = fingerprint::Fingerprint {
        id: Some(1),
        ua_version: 115,
        ua: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36".to_string(),
        language_type: 1,
        languages: "en-US,en;q=0.9".to_string(),
        gmt: "GMT+8".to_string(),
        geography: "China".to_string(),
        geo_tips: 2,
        geo_rule: 0,
        longitude: Some("116.3883".to_string()),
        latitude: Some("39.9289".to_string()),
        radius: Some(100),
        height: Some(1080),
        width: Some(1920),
        fonts_type: 1,
        fonts: Some("Arial, Times New Roman".to_string()),
        font_fingerprint: 1,
        web_rtc: 1,
        web_rtc_local_ip: Some("192.168.1.10".to_string()),
        canvas: 0,
        webgl: 1,
        hardware_acceleration: 1,
        webgl_info: 1,
        audio_context: 1,
        speech_voices: 1,
        media: 1,
        cpu: 8,
        memory: 6,
        do_not_track: 0,
        battery: 1,
        port_scan: 0,
        white_list: Some("80, 443".to_string()),
        created_at: Some("2024-01-01 12:00:00".to_string()),
        updated_at: Some("2024-01-01 12:00:00".to_string()),
        deleted_at: None,
    };

    let result = fingerprint::Fingerprint::insert(fingerprint_info).await;

    println!("{:?}", result);
}

#[tokio::test]
async fn test_fingerprint_update() {
    use cores::models::fingerprint::{self};
    crate::cores::models::load_db().await;

    let fingerprint_info = fingerprint::Fingerprint {
    id: Some(1),
    ua_version: 108,
    ua: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36".to_string(),
    language_type: 1,
    languages: "en-US,en;q=0.9".to_string(),
    gmt: "GMT+8".to_string(),
    geography: "China".to_string(),
    geo_tips: 2,
    geo_rule: 0,
    longitude: Some("116.3883".to_string()),
    latitude: Some("39.9289".to_string()),
    radius: Some(100),
    height: Some(1080),
    width: Some(1920),
    fonts_type: 1,
    fonts: Some("Arial, Times New Roman".to_string()),
    font_fingerprint: 1,
    web_rtc: 1,
    web_rtc_local_ip: Some("192.168.1.10".to_string()),
    canvas: 0,
    webgl: 1,
    hardware_acceleration: 1,
    webgl_info: 1,
    audio_context: 1,
    speech_voices: 1,
    media: 1,
    cpu: 8,
    memory: 6,
    do_not_track: 0,
    battery: 1,
    port_scan: 0,
    white_list: Some("80, 443".to_string()),
    created_at: Some("2024-01-01 12:00:00".to_string()),
    updated_at: Some("2024-01-01 12:00:00".to_string()),
    deleted_at: None,
};

    let result = fingerprint::Fingerprint::update_fingerprints(fingerprint_info).await;

    println!("{:?}", result);
}

#[tokio::test]
async fn test_fingerprint_delete() {
    use cores::models::fingerprint;
    crate::cores::models::load_db().await;

    let result = fingerprint::Fingerprint::delete(1).await.unwrap();

    println!("{:?}", result);
}

#[tokio::test]
async fn test_fingerprint_query() {
    use cores::models::fingerprint;
    crate::cores::models::load_db().await;

    let result = fingerprint::Fingerprint::query_fingerprints(&cores::apis::PageParam {
        page_num: Some(1),
        page_size: Some(10),
    })
    .await
    .unwrap();

    println!("{:?}", result);
}

#[tokio::test]
async fn test_fingerprint_query_by_id() {
    use cores::models::fingerprint;
    crate::cores::models::load_db().await;

    let result = fingerprint::Fingerprint::query_fingerprint_by_id(2)
        .await
        .unwrap();

    println!("{:?}", result);
}
