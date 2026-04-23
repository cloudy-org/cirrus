use std::path::PathBuf;

use log::debug;

use crate::error::{Error, Result};

pub mod error;

#[cfg(target_os = "windows")]
const CLOUDY_FOLDER_NAME: &str = "Cloudy";

#[cfg(not(target_os = "windows"))]
const CLOUDY_FOLDER_NAME: &str = "cloudy";

/// **Linux:** `~/.config/cloudy`
/// 
/// **Windows:** `C:\Users\{user}\AppData\Local\Cloudy`
pub fn get_user_config_cloudy_folder_path() -> Result<PathBuf> {
    debug!("Getting user's local configuration cloudy-org folder...");

    match dirs::config_local_dir() {
        Some(local_config_dir) => Ok(local_config_dir.join(CLOUDY_FOLDER_NAME)),
        None => Err(Error::PathNotFoundForPlatform)
    }
}

/// **Linux:** `~/.cache/cloudy`
/// 
/// **Windows:** `C:\Users\{user}\AppData\Local\Cloudy`
pub fn get_user_cache_cloudy_folder_path() -> Result<PathBuf> {
    debug!("Getting user's local cache cloudy-org folder...");

    match dirs::cache_dir() {
        Some(cache_dir) => Ok(cache_dir.join(CLOUDY_FOLDER_NAME)),
        None => Err(Error::PathNotFoundForPlatform)
    }
}

/// **Linux:** `~/.local/share/cloudy/themes`
/// 
/// **Windows:** `C:\Users\{user}\AppData\Local\Cloudy\themes`
pub fn get_user_cloudy_themes_folder_path() -> Result<PathBuf> {
    debug!("Getting user's cloudy-org theme folder...");

    match dirs::data_local_dir() {
        Some(local_data_dir) => Ok(local_data_dir.join(CLOUDY_FOLDER_NAME).join("themes")),
        None => Err(Error::PathNotFoundForPlatform)
    }
}

/// **Linux:** Follows `$XDG_DATA_DIRS` which would usually default to `/usr/local/share/cloudy/themes` and `/usr/share/cloudy/themes`.
/// 
/// This path should never be created! The package manager will create this path for us, we just read it for discovery.
pub fn get_system_cloudy_themes_folder_paths() -> Result<Vec<PathBuf>> {
    debug!("Getting system cloudy-org folders that could contain themes...");

    #[cfg(target_os = "linux")]
    {
        use xdg::BaseDirectories;

        let xdg_data_dirs = BaseDirectories::new().get_data_dirs();

        return Ok(
            xdg_data_dirs.iter()
                .map(|path_buf| path_buf.join(CLOUDY_FOLDER_NAME).join("themes"))
                .collect()
        );
    }

    #[allow(unreachable_code)]
    Err(Error::PathNotFoundForPlatform)
}