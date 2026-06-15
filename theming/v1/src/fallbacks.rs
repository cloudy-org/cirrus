use crate::{colour::Colour, palette::DEFAULT_ACCENT_HEX};

pub struct ThemeFallbacks {
    pub system_derived_accent_colour: Colour,
}

impl Default for ThemeFallbacks {
    fn default() -> Self {
        Self {
            system_derived_accent_colour: Colour::from_hex(DEFAULT_ACCENT_HEX)
        }
    }
}