use crate::{colour::Colour, pallet::DEFAULT_ACCENT_HEX};

#[derive(Default)]
pub struct ThemeFallbacks {
    pub pallet: ThemePalletFallbacks,
}

pub struct ThemePalletFallbacks {
    pub accent_colour: Colour,
}

impl Default for ThemePalletFallbacks {
    fn default() -> Self {
        Self {
            accent_colour: Colour::from_hex(DEFAULT_ACCENT_HEX)
        }
    }
}