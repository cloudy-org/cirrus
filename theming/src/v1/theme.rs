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
        // let mut default_pallet = ColourPallet {
        //     primary: 0xF5FFFA.into(),
        //     surface: 0xF4C2C2.into(),
        //     text: 0x423939.into(),
        //     accent: 0xFBAED2.into(),
        // };

        let mut default_pallet = ColourPallet::default_dark();

        if let Some(accent_colour) = fallback_accent_colour {
            default_pallet.accent = accent_colour;
        }

        Self {
            // TODO: load theme from user's global theme or else use default ColourPallet.
            is_dark: true,
            pallet: default_pallet,
        }
    }
}

// Retrieves the system theme... (WIP!)
// pub fn get_theme() -> Theme {}