use std::{fs, path::PathBuf};

use cirrus_path::{get_system_cloudy_themes_folder_paths, get_user_cloudy_themes_folder_path};

use crate::{fallbacks::ThemeFallbacks, theme::Theme};

pub fn find_theme_in_system(theme_code_name: String, fallbacks: &ThemeFallbacks) -> Option<Theme> {
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
            Ok(themes_read_dir) => {
                for theme_folder_dir_entry in themes_read_dir {
                    if let Ok(theme_folder_dir_entry) = theme_folder_dir_entry {
                        if theme_folder_dir_entry.file_name().to_string_lossy().to_lowercase() == theme_code_name.to_lowercase() {
                            let theme_path = theme_folder_dir_entry.path();

                            return match Theme::parse_from_path(theme_path, fallbacks) {
                                Ok(theme) => Some(theme),
                                Err(error) => {
                                    log::error!("{}", error);

                                    None
                                },
                            };
                        }
                    }
                }

                continue;
            },
            Err(error) => {
                log::warn!(
                    "Failed to read themes path '{}'! \
                        Skipping this directory... \n\nError: {error}",
                    themes_path.display()
                );

                continue;
            },
        }
    }

    log::warn!(
        "The theme '{theme_code_name}' was not found in the system!"
    );

    return None;
}