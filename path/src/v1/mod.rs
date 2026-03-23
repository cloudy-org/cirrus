use std::path::PathBuf;

use log::debug;

use crate::v1::error::{Error, Result};

pub mod error;

#[cfg(target_os = "windows")]
const CLOUDY_FOLDER_NAME: &str = "Cloudy";

#[cfg(not(target_os = "windows"))]
const CLOUDY_FOLDER_NAME: &str = "cloudy";

pub fn get_user_config_cloudy_folder_path() -> Result<PathBuf> {
    debug!("Getting user's local configuration cloudy-org folder path...");

    match dirs::config_local_dir() {
        Some(local_config_dir) => Ok(local_config_dir.join(CLOUDY_FOLDER_NAME)),
        None => Err(Error::PathNotFoundForPlatform)
    }
}

pub fn get_user_cache_cloudy_folder_path() -> Result<PathBuf> {
    debug!("Getting user's local cache cloudy-org folder path...");

    match dirs::cache_dir() {
        Some(cache_dir) => Ok(cache_dir.join(CLOUDY_FOLDER_NAME)),
        None => Err(Error::PathNotFoundForPlatform)
    }
}