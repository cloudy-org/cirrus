use std::path::PathBuf;

use log::debug;

use crate::v1::error::{Error, Result};

pub mod error;

pub fn get_user_config_dir_path(app_name: &str) -> Result<PathBuf> {
    debug!("Finding the user's local configuration path...");

    match dirs::config_local_dir() {
        Some(local_config_dir) => Ok(local_config_dir.join("cloudy").join(app_name)),
        None => Err(Error::DirNotFoundForPlatform)
    }
}