use std::{fs, io::ErrorKind, path::PathBuf};

use cirrus_path::{get_system_cloudy_themes_folder_paths, get_user_cloudy_themes_folder_path};

use crate::{fallbacks::ThemePalletFallbacks, theme::Theme};

pub fn find_theme_in_system(theme_code_name: String, pallet_fallbacks: &ThemePalletFallbacks) -> Option<Theme> {
    log::debug!("Searching for the cloudy-org theme '{}' on this system...", theme_code_name);

    let mut themes_paths: Vec<PathBuf> = Vec::new();

    if let Ok(user_themes_path) = get_user_cloudy_themes_folder_path() {
        themes_paths.push(user_themes_path);
    }

    themes_paths.extend(
        get_system_cloudy_themes_folder_paths().unwrap_or_default()
    );

    for themes_path in themes_paths {
        match fs::read_dir(&themes_path) {
            Ok(read_dir) => {
                for theme_pack_folder_entry in read_dir {
                    match theme_pack_folder_entry {
                        Ok(theme_pack_folder) => {
                            let theme_path = theme_pack_folder.path()
                                .join(theme_code_name.to_lowercase());

                            if theme_path.is_dir() {
                                return match Theme::parse_from_path(theme_path, pallet_fallbacks) {
                                    Ok(theme) => Some(theme),
                                    Err(error) => {
                                        log::error!("{}", error);

                                        None
                                    },
                                };
                            }

                            continue;
                        },
                        Err(error) => {
                            log::warn!(
                                "Cannot fetch theme pack in directory '{}'! Error: {error}",
                                themes_path.display()
                            );

                            continue;
                        },
                    }
                }
            },
            Err(error) => {
                let error_message = format!(
                    "Cannot search the directory '{}' for cloudy-org themes! Error: {error}",
                    themes_path.display()
                );

                // It is very common for the other theme directories to not exist so 
                // if the error is an "No such file or directory (os error 2)" error 
                // just log it at debug level instead of warning then continue.
                if error.kind() == ErrorKind::NotFound {
                    log::debug!("{}", error_message);
                    continue;
                }

                log::warn!("{}", error_message);

                continue;
            },
        }
    }

    log::warn!(
        "The theme '{theme_code_name}' was not found in the system!"
    );

    return None;
}