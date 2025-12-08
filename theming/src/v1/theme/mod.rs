mod config;

use toml::Table;
use std::{env, fs};

use cirrus_error::v1::error::CError;
use cirrus_path::v1::get_user_config_cloudy_folder_path;

use crate::v1::{colour::Colour, error::Error, pallet::{ColourPallet, DEFAULT_ACCENT_HEX}};

#[derive(Clone)]
pub struct Theme {
    pub pallet: ColourPallet
}

// TODO: Document Theme struct
impl Theme {
    /// If the user's system accent colour cannot be derived we fallback to `fallback_accent_colour`.
    pub fn new(
        fallback_accent_colour: Option<Colour>
    ) -> Self {
        // let default_pallet = ColourPallet {
        //     primary: 0xF5FFFA.into(),
        //     surface: 0xF4C2C2.into(),
        //     text: 0x423939.into(),
        //     accent: 0xFBAED2.into(),
        // };

        let fallback_accent_colour = fallback_accent_colour.unwrap_or(
            Colour::from_hex(DEFAULT_ACCENT_HEX)
        );

        let fallback_config_theme = Theme {
            pallet: ColourPallet {
                accent: fallback_accent_colour,
                ..ColourPallet::default_dark()
            }
        };

        let config_theme = match Self::get_theme_from_config(fallback_accent_colour) {
            Ok(Some(theme)) => theme,
            Ok(None) => {
                log::warn!("No theme.toml file found, cirrus may use a default theme!");

                fallback_config_theme
            }
            Err(error) => {
                log::error!(
                    "Cirrus failed to get theme from theme.toml! Error: {}",
                    error.actual_error().unwrap_or_default()
                );

                fallback_config_theme
            },
        };

        let mut theme = config_theme;

        if let Some(pallet) = Self::get_pallet_from_env() {
            theme.pallet = pallet;
        }

        theme
    }

    fn get_pallet_from_env() -> Option<ColourPallet> {
        // TODO: this will be upgraded to support multiple predefined cirrus themes in the future 
        // (E.g: "CTK_THEME=candy_crush"). For now we'll just support our default dark and light themes.
        match env::var("CTK_THEME") {
            Ok(theme_name) => match theme_name.to_lowercase().as_str() {
                "light" => Some(ColourPallet::default_light()),
                "dark" => Some(ColourPallet::default_dark()),
                _ => {
                    log::warn!(
                        "Alternative themes are not supported yet in cirrus, defaulting to fallback..."
                    );

                    None
                }
            },
            Err(_) => None, // also errors when there's no env var btw
        }
    }

    fn get_theme_from_config(fallback_accent_colour: Colour) -> Result<Option<Self>, Error> {
        let theme_path = get_user_config_cloudy_folder_path()
            .map_err(
                |error| Error::FailedToFindThemeToml(
                    error.actual_error().unwrap_or_default() // actual error will always be there so it should never default...
                    // ..but eh, just in case — let's be a little panic safe in case this behaviour changes
                )
            )?.join("theme.toml");

        match theme_path.exists() {
            true => {
                let toml_string = fs::read_to_string(theme_path)
                    .map_err(|error| Error::FailedToReadThemeToml(error.to_string()))?;

                let generic_theme_table: Table = toml::from_str(&toml_string)
                    .map_err(|error| Error::FailedToReadThemeToml(error.to_string()))?;

                let theme_version = generic_theme_table.get("version")
                    .ok_or(Error::FailedToReadThemeToml(
                        String::from("Could not find 'version' key inside theme file!")
                    ))?;

                match theme_version.as_integer() {
                    Some(1) => Ok(Some(config::v1::parse(&toml_string, fallback_accent_colour)?)),
                    _ => Err(
                        Error::FailedToReadThemeToml(
                            String::from(
                                "Unsupported theme version! This current version of \
                                    cirrus does not support this version of theme.toml!"
                            )
                        )
                    ),
                }
            }
            false => Ok(None)
        }
    }
}