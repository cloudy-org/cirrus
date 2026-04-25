use std::{env, fs, path::PathBuf};

use toml::Table;
use cirrus_path::get_user_config_cloudy_folder_path;

use crate::{colour::Colour, error::{Error, Result}, fallbacks::ThemeFallbacks, manager::origin::ThemeOrigin, pallet::DEFAULT_ACCENT_HEX, system::find_theme_in_system, theme::Theme};

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
            theme: Theme::default_dark(),
            fallbacks: ThemeFallbacks::default(),
            origin: None,
        }
    }
}

impl ThemeManager {
    pub fn set_fallbacks(mut self, fallbacks: ThemeFallbacks) -> Self {
        self.fallbacks = fallbacks;

        self
    }

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

            if let Some(found_theme) = find_theme_in_system(theme_name, &self.fallbacks) {
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
            self.fallbacks.accent_colour = Colour::from_hex(DEFAULT_ACCENT_HEX);

            match find_theme_from_config(config_path, &self.fallbacks) {
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

fn find_theme_from_config(config_path: PathBuf, fallbacks: &ThemeFallbacks) -> Result<Option<Theme>, Error> {
    log::debug!("Checking global config toml for set theme...");

    // TODO: we should use the cirrus_config crate when it get's support for this global config. 
    let toml_string = fs::read_to_string(config_path)
        .map_err(|error| Error::GlobalConfigParseFailure { error: error.to_string() })?;

    let generic_config_table: Table = toml::from_str(&toml_string)
        .map_err(|error| Error::GlobalConfigParseFailure { error: error.to_string() })?;

    if let Some(theme_code_name_value) = generic_config_table.get("theme") {
        if let Some(theme_code_name) = theme_code_name_value.as_str() {
            return Ok(
                find_theme_in_system(theme_code_name.to_string(), fallbacks)
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