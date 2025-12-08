use std::path::PathBuf;

use cirrus_error::v1::error::CError;
use log::debug;

use crate::v1::error::{Error, Result};

pub mod error;

pub fn get_user_config_cloudy_folder_path() -> Result<PathBuf, Box<dyn CError>> {
    debug!("Finding the user's local configuration cloudy-org folder path...");

    match dirs::config_local_dir() {
        Some(local_config_dir) => Ok(local_config_dir.join("cloudy")),
        None => Err(Error::DirNotFoundForPlatform.into())
    }
}