use serde::{Deserialize};

use crate::{colour::Colour, error::Error, fallbacks::ThemeFallbacks, palette::{ColourPalette, TRANSPARENT_HEX}, theme::Theme};

#[derive(Deserialize)]
pub struct ThemeConfigV1 {
    #[allow(dead_code)]
    version: i8,
    dark_mode: bool,
    metadata: Metadata,
    palette: ThemePalette,
    // #[serde(default)]
    // features: ThemeFeatures,
}

#[derive(Deserialize)]
struct Metadata {
    pub name: String,
}

#[derive(Deserialize)]
struct ThemePalette {
    #[serde(default)]
    pub primary: Option<String>,
    #[serde(default)]
    pub interactive: Option<String>,
    #[serde(default)]
    pub surface: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub accent: Option<String>,
}

pub fn parse(toml_string: &str, fallbacks: &ThemeFallbacks) -> Result<Theme, Error> {
    let theme_config: ThemeConfigV1 = toml::from_str(&toml_string)
        .map_err(|error| Error::ThemeTomlParseFailure { error: error.to_string() })?;

    let is_dark = theme_config.dark_mode;
    // let theme_features = theme_config.features;
    let theme_palette = theme_config.palette;

    let transparent_colour = Colour::from_hex(TRANSPARENT_HEX);

    let primary_colour: Colour = match theme_palette.primary {
        Some(hex_string) => Colour::try_from(hex_string)?,
        None => transparent_colour,
    };

    let interactive_colour: Colour = match theme_palette.interactive {
        Some(hex_string) => Colour::try_from(hex_string)?,
        None => transparent_colour,
    };

    let surface_colour = match theme_palette.surface {
        Some(hex_string) => Colour::try_from(hex_string)?,
        None => transparent_colour,
    };

    let text_colour = match theme_palette.text {
        Some(hex_string) => Colour::try_from(hex_string)?,
        None => match is_dark {
            true => Colour::from_hex(0xffffff),
            false => Colour::from_hex(0x000000)
        },
    };

    let accent_colour = match theme_palette.accent {
        Some(hex_string) => Colour::try_from(hex_string)?,
        None => fallbacks.system_derived_accent_colour,
    };

    Ok(
        Theme {
            name: theme_config.metadata.name,
            // features: Features {
            //     derive_accent_from_system: theme_features.derive_accent_from_system,
            // },
            palette: ColourPalette {
                is_dark: theme_config.dark_mode,
                primary: primary_colour,
                interactive: interactive_colour,
                surface: surface_colour,
                text: text_colour,
                accent: accent_colour
            },
        }
    )
}

#[derive(Deserialize, Default)]
struct ThemeFeatures {
    pub derive_accent_from_system: bool,
}