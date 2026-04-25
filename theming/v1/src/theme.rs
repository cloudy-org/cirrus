use std::{fs, path::PathBuf};

use toml::Table;

use crate::{config, error::{Error, Result}, fallbacks::{ThemeFallbacks, ThemePalletFallbacks}, pallet::ColourPallet};

// TODO: Document Theme struct
#[derive(Clone)]
pub struct Theme {
    pub name: String,
    pub pallet: ColourPallet
}

impl Theme {
    pub fn default_dark(fallbacks: &ThemeFallbacks) -> Self {
        Self {
            name: String::from("Dark"),
            pallet: ColourPallet::default_dark(fallbacks.pallet.accent_colour),
        }
    }

    pub fn default_light(fallbacks: &ThemeFallbacks) -> Self {
        Self {
            name: String::from("Light"),
            pallet: ColourPallet::default_light(fallbacks.pallet.accent_colour),
        }
    }

    pub(crate) fn parse_from_path(theme_path: PathBuf, pallet_fallbacks: &ThemePalletFallbacks) -> Result<Self> {
        log::debug!("Parsing theme from path '{}'...", &theme_path.display());

        let theme_code_name = theme_path.file_name()
            .ok_or_else(|| Error::PathNotATheme { path: theme_path.clone() })?
            .to_string_lossy()
            .to_string();

        let theme_toml_path = theme_path.join("theme.toml");

        log::debug!("Reading from '{}'...", theme_toml_path.display());

        let toml_string = fs::read_to_string(&theme_toml_path)
            .map_err(
                |error| Error::ThemeTomlParseFailure {
                    error: format!(
                        "Failed to read '{theme_code_name}' theme.toml! \n\nError: {}",
                        error.to_string()
                    )
                }
            )?;

        log::debug!("Parsing '{theme_code_name}'s theme toml file...");

        // TODO: Hey, right now the config.toml file get's deserialized twice (here and in 'config::v1::parse()'), 
        // it would be cool if someone could find out how to share the `generic_theme_table` with 'config::v1::parse()' 
        // but still end up with a `ThemeConfigV1`. In other words, somehow turn `Table` into `ThemeConfigV1`.

        let generic_theme_table = toml::from_str::<Table>(&toml_string)
            .map_err(
                |error| Error::ThemeTomlParseFailure {
                    error: format!(
                        "Failed to parse '{theme_code_name}' toml into toml.rs generic table! \n\nError: {}",
                        error.to_string()
                    )
                }
            )?;

        match generic_theme_table.get("version") {
            Some(theme_version) => {
                match theme_version.as_integer() {
                    Some(1) => Ok(config::v1::parse(&toml_string, pallet_fallbacks)?),
                    _ => Err(Error::ThemeTomlUnsupported { version: theme_version.to_string() }),
                }
            },
            None => Err(
                Error::ThemeTomlNoVersionKey {
                    theme_code_name, theme_toml_path
                }
            ),
        }
    }
}