use std::path::PathBuf;

pub mod processor;
pub mod request;
pub mod requests;

#[cfg(not(windows))]
mod not_win_imports {
    pub use aes::Aes128;
    pub use block_modes::{BlockMode, Cbc};
    pub use block_padding::Pkcs7;
}

pub async fn init_location(locations: Vec<PathBuf>) -> Result<(), std::io::Error> {
    let cache_location = states::tauri::get_app_cache_location().await;
    let cache_location = if let Some(path) = cache_location {
        path
    } else {
        std::env::current_dir()?
    };

    for location in locations {
        let setting_file_location = cache_location.join(location);

        if !setting_file_location.exists() {
            std::fs::create_dir_all(setting_file_location)?;
        }
    }

    Ok(())
}
