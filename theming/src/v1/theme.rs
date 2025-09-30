use std::env;

use crate::v1::{colour::Colour, pallet::ColourPallet};

#[derive(Clone)]
pub struct Theme {
    pub is_dark: bool,

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

        let mut is_dark = true;

        // TODO: 'default_pallet' will soon be a theme from the 'theme.toml' file.
        let default_pallet = ColourPallet::default_dark();

        // TODO: this will be upgraded to support multiple predefined cirrus themes in the future 
        // (E.g: "CTK_THEME=candy_crush"). For now we'll just support our default dark and light themes.
        let mut pallet = match env::var("CTK_THEME") {
            Ok(theme_name) => match theme_name.to_lowercase().as_str() {
                "dark" => {
                    is_dark = true;
                    ColourPallet::default_dark()
                },
                "light" => {
                    is_dark = false;
                    ColourPallet::default_light()
                },
                _ => default_pallet
            },
            Err(_) => default_pallet,
        };

        // TODO: use system accent colour and only fallback 
        // to app's accent colour when those efforts have failed.
        if let Some(accent_colour) = fallback_accent_colour {
            pallet.accent = accent_colour;
        }

        Self {
            is_dark, pallet,
        }
    }
}

// Retrieves the system theme... (WIP!)
// pub fn get_theme() -> Theme {}