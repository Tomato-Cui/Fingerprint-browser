pub mod chrome_extension_scrapy {
    use regex::Regex;
    use reqwest::Client;
    use serde_json::{json, Value};

    pub fn parse_url_get_extension_uuid(url: &str) -> Result<&str, anyhow::Error> {
        let prefix = "chromewebstore.google.com/detail";

        if !url.contains(prefix) {
            return Err(anyhow::anyhow!("the url invalid."));
        }

        let re = Regex::new(r"/detail/[^/]+/([a-z0-9]{32})")?;

        if let Some(captures) = re.captures(url) {
            if let Some(extension_uuid) = captures.get(1) {
                return Ok(extension_uuid.as_str());
            }
        }
        return Err(anyhow::anyhow!("The url invalid."));
    }

    pub fn extension_download_url(chrome_version: &str, extension_uuid: &str) -> String {
        format!("https://clients2.google.com/service/update2/crx?response=redirect&prodversion={}&acceptformat=crx2%2Ccrx3&x=id%3D{}%26uc", chrome_version,extension_uuid)
    }

    pub async fn extension_detail_by_url(url: &str) -> Result<Value, anyhow::Error> {
        let response = Client::new().get(url).send().await?;

        let response_text = response.text().await?;

        let mut json_value = json!({});

        let extension_uuid = parse_url_get_extension_uuid(url)?;
        json_value["extension_uuid"] = json!(extension_uuid);

        let title_re = Regex::new(r#"<h1 [^>]*>(.*?)</h1>"#).unwrap();
        if let Some(captures) = title_re.captures(&response_text) {
            if let Some(extension_title) = captures.get(1) {
                json_value["extension_title"] = json!(extension_title.as_str());
            }
        }

        let avatar_re =
        Regex::new(r#"<div[^>]*class="KgGEHd"[^>]*><img\ssrc=\"(.*?)\"\s\S.*</div>"#).unwrap();
        if let Some(captures) = avatar_re.captures(&response_text) {
            if let Some(extension_avatar) = captures.get(1) {
                json_value["extension_avatar"] = json!(extension_avatar.as_str());
            }
        }

        let description_re =
            Regex::new(r#"<div[^>]*class="RNnO5e"[^>]*>([\s\S]*?)</div>"#).unwrap();
        if let Some(div_cap) = description_re.captures(&response_text) {
            if let Some(div_content) = div_cap.get(1) {
                json_value["extension_description"] = json!(div_content.as_str());
            }
        }

        Ok(json_value)
    }

    #[test]
    fn test_parse_url_get_extension_uuid() {
        let extension_uuid = parse_url_get_extension_uuid("https://chromewebstore.google.com/detail/sponsorblock-for-youtube/mnjggcdmjocbbbhaepdhchncahnbgone?hl=zh-CN&utm_source=ext_sidebar");

        println!("{:?}", extension_uuid);
    }

    #[tokio::test]
    async fn test_extension_detail_by_url() {
        let urls = vec![
            // "https://chromewebstore.google.com/detail/evernote-web-clipper/pioclpoplcdbaefihamjohnefbikjilc",
            // "https://chromewebstore.google.com/detail/1930/mbcibgkijjmnhbbheafplbapiacgkebe?hl=zh-CN&utm_source=ext_sidebar",
            // "https://chromewebstore.google.com/detail/socialiq-influencer-marke/edpcocadldfbbpllhfkfcebnpigleamn?hl=zh-CN&utm_source=ext_sidebar",
            // "https://chromewebstore.google.com/detail/happy-dog-virtual-pet-for/cdoblkdcnbcdlcfklmbmkapbekgfbijp?hl=zh-CN&utm_source=ext_sidebar",
            // "https://chromewebstore.google.com/detail/pinball-space-adventure-g/iibghlclocgeljpbimnneepjceaiepkj?hl=zh-CN&utm_source=ext_sidebar"
            // "https://chromewebstore.google.com/detail/solid-devtools/kmcfjchnmmaeeagadbhoofajiopoceel?hl=zh-CN&utm_source=ext_sidebar"
            "https://chromewebstore.google.com/detail/axe-devtools-web-accessib/lhdoppojpmngadmnindnejefpokejbdd?hl=zh-CN&utm_source=ext_sidebar",
        ];

        for url in urls {
            let res = extension_detail_by_url(url).await.unwrap();

            println!("> {:?}", res);
        }
    }

    #[test]
    fn test_extension_download_url() {
        let extension_uuid = parse_url_get_extension_uuid("https://chromewebstore.google.com/detail/sponsorblock-for-youtube/mnjggcdmjocbbbhaepdhchncahnbgone?hl=zh-CN&utm_source=ext_sidebar").unwrap();

        let url = extension_download_url("131.0.6778.205", extension_uuid);
        println!("{:?}", url);
    }
}
