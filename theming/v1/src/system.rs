use std::path::PathBuf;

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
        let theme_path = themes_path.join(theme_code_name.to_lowercase());

        if theme_path.is_dir() {
            return match Theme::parse_from_path(theme_path, pallet_fallbacks) {
                Ok(theme) => Some(theme),
                Err(error) => {
                    log::error!("{}", error);

                    None
                },
            };
        }
    }

    log::warn!(
        "The theme '{theme_code_name}' was not found in the system!"
    );

    return None;
}