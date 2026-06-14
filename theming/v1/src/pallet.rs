use crate::{colour::Colour};

pub(crate) static TRANSPARENT_HEX: u32 = 0xFF000000;
pub(crate) static DEFAULT_ACCENT_HEX: u32 = 0x7afff8;

#[derive(Clone)]
pub struct ColourPallet {
    /// Is this colour pallet meant for dark mode?
    pub is_dark: bool,
    /// Primary colour should be used for background colour (e.g: app window canvas).
    pub primary: Colour,
    pub interactive: Colour,
    // TODO: correct this
    /// Surface should be lighter than primary by a subtle difference.
    /// 
    /// Surface is a supporting colour that blends with primary to create a secondary colour that is then 
    /// used on elevated layers to give subtle emphasis; like containers inside a section, cards, toolbars or 
    /// backgrounds inside a panel. The blended secondary colour is also used on some interactable widgets.
    pub surface: Colour,
    /// Foreground text colour.
    pub text: Colour,
    /// Accent is the strongest emphasis colour. It's used in interactive 
    /// elements like buttons, sliders, highlights and it's also sometimes used 
    /// as the stroke colour when drawing certain patterns and in loading animations.
    pub accent: Colour,
}

impl ColourPallet {
    pub(crate) fn default_dark(accent_colour: Colour) -> Self {
        Self {
            is_dark: true,
            primary: Colour::from_hex(0x0A0A0A),
            interactive: Colour::from_hex(0x383838),
            surface: Colour::from_hex(0x3C3939),
            text: Colour::from_hex(0xb5b5b5),
            accent: accent_colour
        }
    }

    pub(crate) fn default_light(accent_colour: Colour) -> Self {
        Self {
            is_dark: false,
            primary: Colour::from_hex(0xEDFFFF),
            interactive: Colour::from_hex(0xF0F2F2),
            surface: Colour::from_hex(0xD1E6E6),
            text: Colour::from_hex(0x242424),
            accent: accent_colour
        }
    }
}