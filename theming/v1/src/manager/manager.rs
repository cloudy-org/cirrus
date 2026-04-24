use std::{env, fs, path::PathBuf};

use cirrus_path::{get_system_cloudy_themes_folder_paths, get_user_cloudy_themes_folder_path, get_user_config_cloudy_folder_path};
use toml::Table;

use crate::{colour::Colour, error::{Error, Result}, manager::{origin::ThemeOrigin}, pallet::DEFAULT_ACCENT_HEX, theme::Theme};

/// ⚠️ Keep in mind this struct is unstable and may change soon with breaking changes.
pub struct ThemeManager {
    pub theme: Theme,

    // might make this public soon
    origin: Option<ThemeOrigin>
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self {
            theme: Theme::default_dark(),
            origin: None
        }
    }
}

impl ThemeManager {
    pub fn get_theme_from_env(mut self) -> Self {
        // TODO: this will be upgraded to support multiple predefined cirrus themes in the future 
        // (E.g: "CTK_THEME=candy_crush"). For now we'll just support our default dark and light themes.
        if let Ok(theme_name) = env::var("CTK_THEME") {
            log::debug!("Getting theme from environment variable...");

            let theme_name = theme_name.to_lowercase();

            if "light" == theme_name {
                self.theme = Theme::default_light();
                self.origin = Some(ThemeOrigin::EnvVar);
                return self;
            }

            if "dark" == theme_name {
                self.theme = Theme::default_dark();
                self.origin = Some(ThemeOrigin::EnvVar);
                return self;
            }

            log::warn!(
                "Alternative themes are not supported yet in cirrus via environment variables!"
            );
        }

        return self;
    }

    /// Fetches set theme from cloudy-org global config (~/.config/cloudy/config.toml) 
    /// and your environment (e.g: KDE / Gnome accent colour).
    pub fn get_theme_from_system(mut self) -> Self {
        if self.origin.is_some() {
            return self;
        }

        log::debug!("Getting theme from system...");

        if let Ok(config_path) = get_user_config_cloudy_folder_path() {
            let config_path = config_path.join("config.toml"); 

            if !config_path.exists() {
                return self;
            }

            // TODO: Fetch system accent colour.
            let system_accent_colour = Colour::from_hex(DEFAULT_ACCENT_HEX);

            match find_theme_from_config(config_path, system_accent_colour) {
                Ok(Some(theme)) => {
                    self.theme = theme;
                    self.origin = Some(ThemeOrigin::Config);
                },
                Ok(None) => log::debug!("No theme was set in the config."),
                Err(error) => log::error!(
                    "Failed to get theme from config! \n\nError: {error}"
                ),
            }
        } 

        self
    }
}

fn find_theme_from_config(config_path: PathBuf, system_accent_colour: Colour) -> Result<Option<Theme>, Error> {
    log::debug!("Checking global config toml for set theme...");

    // TODO: we should use the cirrus_config crate when it get's support for this global config. 
    let toml_string = fs::read_to_string(config_path)
        .map_err(|error| Error::GlobalConfigParseFailure { error: error.to_string() })?;

    let generic_config_table: Table = toml::from_str(&toml_string)
        .map_err(|error| Error::GlobalConfigParseFailure { error: error.to_string() })?;

    if let Some(theme_code_name_value) = generic_config_table.get("theme") {
        if let Some(theme_code_name) = theme_code_name_value.as_str() {
            return Ok(
                find_theme_in_system(theme_code_name.to_string(), system_accent_colour)
            );
        }

        return Err(
            Error::ThemeTomlParseFailure {
                error: "The 'theme' key in the global config must be a string!".into()
            }
        );
    };

    Ok(None)
}

fn find_theme_in_system(theme_code_name: String, system_accent_colour: Colour) -> Option<Theme> {
    log::debug!("Searching for the cloudy-org theme '{}' on this system...", theme_code_name);

    // TODO: improve my testing code

    match get_user_cloudy_themes_folder_path() {
        Ok(user_themes_path) => {
            match fs::read_dir(&user_themes_path) {
                Ok(theme_folders) => {
                    for theme_folder in theme_folders {
                        if let Ok(theme_folder) = theme_folder {
                            if theme_folder.file_name().to_string_lossy().to_lowercase() == theme_code_name.to_lowercase() {
                                let theme_path = theme_folder.path();

                                return match Theme::parse_from_path(theme_path, system_accent_colour) {
                                    Ok(theme) => Some(theme),
                                    Err(error) => {
                                        log::error!("{}", error);

                                        None
                                    },
                                };
                            }
                        }
                    }
                },
                Err(error) => {
                    log::warn!(
                        "Failed to read themes path '{}'! \
                            Skipping this directory... \n\nError: {error}",
                        user_themes_path.display()
                    );
                },
            }
        },
        Err(error) => {
            log::error!(
                "Could not get local user themes path! \n\nError: {error}"
            );
        },
    }

    let system_themes_paths = get_system_cloudy_themes_folder_paths()
        .unwrap_or_default();

    for system_theme_path in system_themes_paths {
        match fs::read_dir(&system_theme_path) {
            Ok(theme_folders) => {
                for theme_folder in theme_folders {
                    if let Ok(theme_folder) = theme_folder {
                        if theme_folder.file_name().to_string_lossy().to_lowercase() == theme_code_name.to_lowercase() {
                            let theme_path = theme_folder.path();

                            return match Theme::parse_from_path(theme_path, system_accent_colour) {
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
                    system_theme_path.display()
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