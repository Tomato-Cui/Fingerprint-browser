pub mod processor;
pub mod requests;

use lp_states::config::APP_DATA;
use std::path::PathBuf;
pub async fn init_location(locations: Vec<PathBuf>) -> Result<(), std::io::Error> {
    let cache_location = APP_DATA.clone();

    for location in locations {
        let setting_file_location = cache_location.join(location);

        if !setting_file_location.exists() {
            std::fs::create_dir_all(setting_file_location)?;
        }
    }

    Ok(())
}
