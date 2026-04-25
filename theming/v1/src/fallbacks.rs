use crate::{colour::Colour, pallet::DEFAULT_ACCENT_HEX};

pub struct ThemeFallbacks {
    pub accent_colour: Colour
}

impl Default for ThemeFallbacks {
    fn default() -> Self {
        Self {
            accent_colour: Colour::from_hex(DEFAULT_ACCENT_HEX)
        }
    }
}