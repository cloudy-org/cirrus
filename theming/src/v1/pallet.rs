use crate::v1::colour::Colour;

pub static DEFAULT_ACCENT_HEX: u32 = 0x7afff8;
pub static TRANSPARENT_HEX: u32 = 0xFF000000;

#[derive(Clone)]
pub struct ColourPallet {
    /// Is this colour pallet meant for dark mode?
    pub is_dark: bool,
    /// Primary colour should be used for background colour (e.g: app window canvas).
    pub primary: Colour,
    pub interactive: Colour,
    /// Surface should be lighter than primary by a subtle difference.
    /// 
    /// Surface is a supporting colour that blends with primary to create a secondary colour that is then 
    /// used on elevated layers to give subtle emphasis; like containers inside a section, cards, toolbars or 
    /// backgrounds inside a panel.
    pub surface: Colour,
    /// Foreground text colour.
    pub text: Colour,
    /// Accent is the strongest emphasis colour. It's used in interactive 
    /// elements like buttons, sliders, highlights and it's also sometimes used
    /// as the stroke colour when drawing certain patterns and in loading animations.
    pub accent: Colour,
}

impl ColourPallet {
    pub fn default_dark() -> Self {
        Self {
            is_dark: true,
            primary: Colour::from_hex(0x0A0A0A),
            interactive: Colour::from_hex(0x2E2E2E),
            surface: Colour::from_hex(0x3C3939),
            text: Colour::from_hex(0xb5b5b5),
            accent: Colour::from_hex(DEFAULT_ACCENT_HEX)
        }
    }

    pub fn default_light() -> Self {
        Self {
            is_dark: false,
            primary: Colour::from_hex(0xD5EBEB),
            interactive: Colour::from_hex(0xA6C4E3),
            surface: Colour::from_hex(0xB8E2FF),
            text: Colour::from_hex(0x242424),
            accent: Colour::from_hex(DEFAULT_ACCENT_HEX)
        }
    }
}