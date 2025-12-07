use serde::{Deserialize};

use crate::v1::{colour::Colour, error::Error, pallet::ColourPallet, theme::Theme};

#[derive(Deserialize)]
pub struct ThemeConfigV1 {
    #[warn(dead_code)]
    version: i8,
    dark_mode: bool,
    pallet: ThemePalletV1
}

#[derive(Deserialize)]
struct ThemePalletV1 {
    #[serde(default)]
    pub primary: Option<String>,
    #[serde(default)]
    pub surface: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub accent: Option<String>,
}

pub fn parse(toml_string: &str, fallback_colour_pallet: ColourPallet) -> Result<Theme, Error> {
    let theme_config: ThemeConfigV1 = toml::from_str(&toml_string)
        .map_err(|error| Error::FailedToReadThemeToml(error.to_string()))?;

    let theme_pallet = theme_config.pallet;

    let primary_colour: Colour = match theme_pallet.primary {
        Some(hex_string) => Colour::try_from(hex_string)?,
        None => fallback_colour_pallet.primary,
    };

    let surface_colour = match theme_pallet.surface {
        Some(hex_string) => Colour::try_from(hex_string)?,
        None => fallback_colour_pallet.surface,
    };

    let text_colour = match theme_pallet.text {
        Some(hex_string) => Colour::try_from(hex_string)?,
        None => fallback_colour_pallet.text,
    };

    let accent_colour = match theme_pallet.accent {
        Some(hex_string) => Colour::try_from(hex_string)?,
        None => fallback_colour_pallet.accent,
    };

    Ok(
        Theme {
            pallet: ColourPallet {
                is_dark: theme_config.dark_mode,
                primary: primary_colour,
                surface: surface_colour,
                text: text_colour,
                accent: accent_colour
            },
        }
    )
}