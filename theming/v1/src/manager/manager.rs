use std::{env, fs, path::PathBuf};

use toml::Table;
use cirrus_path::get_user_config_cloudy_folder_path;

use crate::{colour::Colour, error::{Error, Result}, fallbacks::{ThemeFallbacks, ThemePalletFallbacks}, manager::origin::ThemeOrigin, pallet::DEFAULT_ACCENT_HEX, system::find_theme_in_system, theme::Theme};

/// ⚠️ Keep in mind this struct is unstable and may change soon with breaking changes.
pub struct ThemeManager {
    pub theme: Theme,

    fallbacks: ThemeFallbacks,
    // might make this public soon
    origin: Option<ThemeOrigin>,
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self {
            theme: Theme::default_dark(&ThemeFallbacks::default()),
            fallbacks: ThemeFallbacks::default(),
            origin: None
        }
    }
}

impl ThemeManager {
    pub fn default_with_fallbacks(fallbacks: ThemeFallbacks) -> Self {
        Self {
            theme: Theme::default_dark(&fallbacks),
            fallbacks: fallbacks,
            origin: None,
        }
    }

    pub fn get_theme_from_env(mut self) -> Self {
        // TODO: this will be upgraded to support multiple predefined cirrus themes in the future 
        // (E.g: "CTK_THEME=candy_crush"). For now we'll just support our default dark and light themes.
        if let Ok(theme_name) = env::var("CTK_THEME") {
            log::debug!("Getting theme from environment variable...");

            let theme_fallbacks = &self.fallbacks;

            let theme: Option<Theme> = match theme_name.to_lowercase().as_str() {
                "dark" => Some(Theme::default_dark(theme_fallbacks)),
                "light" => Some(Theme::default_light(theme_fallbacks)),
                theme_code_name => find_theme_in_system(
                    theme_code_name.to_string(),
                    &theme_fallbacks.pallet
                )
            };

            if let Some(found_theme) = theme {
                self.theme = found_theme;
                self.origin = Some(ThemeOrigin::EnvVar);
                return self;
            }
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
            self.fallbacks.pallet.accent_colour = Colour::from_hex(DEFAULT_ACCENT_HEX);

            match find_theme_from_config(config_path, &self.fallbacks.pallet) {
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

fn find_theme_from_config(config_path: PathBuf, pallet_fallbacks: &ThemePalletFallbacks) -> Result<Option<Theme>, Error> {
    log::debug!("Checking global config toml for set theme...");

    // TODO: we should use the cirrus_config crate when it get's support for this global config. 
    let toml_string = fs::read_to_string(config_path)
        .map_err(|error| Error::GlobalConfigParseFailure { error: error.to_string() })?;

    let generic_config_table: Table = toml::from_str(&toml_string)
        .map_err(|error| Error::GlobalConfigParseFailure { error: error.to_string() })?;

    if let Some(theme_code_name_value) = generic_config_table.get("theme") {
        if let Some(theme_code_name) = theme_code_name_value.as_str() {
            return Ok(
                find_theme_in_system(theme_code_name.to_string(), pallet_fallbacks)
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