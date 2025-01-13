pub mod config;
pub mod database;
pub mod encryption;
pub mod time;
pub mod util;

#[cfg(not(windows))]
mod not_win_imports {
    pub use aes::Aes128;
    pub use block_modes::{BlockMode, Cbc};
    pub use block_padding::Pkcs7;
}

#[cfg(target_os = "windows")]
pub(crate) mod win_imports {
    pub use aes_gcm::{
        aead::{Aead, KeyInit},
        Aes256Gcm, Key, Nonce,
    };
    pub use rand::RngCore;
    extern crate winapi;

    pub use std::ptr;
    pub use winapi::shared::minwindef::DWORD;
    pub use winapi::um::dpapi::CryptUnprotectData;
    pub use winapi::um::wincrypt::DATA_BLOB;
}
