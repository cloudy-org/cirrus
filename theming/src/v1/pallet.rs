use crate::v1::colour::Colour;

static DEFAULT_ACCENT_HEX: u32 = 0x7afff8;

#[derive(Clone)]
pub struct ColourPallet {
    /// Primary colour should be used for background colour (e.g: app window canvas).
    pub primary: Colour,
    /// This is a secondary background colour that can be used on widgets or sections 
    /// that require extra emphasis than primary. Like a sidebar or floating window.
    pub secondary: Colour,
    /// Subtle neutral layer colour that sits above the primary or secondary colour. Should be 
    /// used for elevated layers like containers inside a section, like cards, toolbars or backgrounds inside a panel.
    pub surface: Colour,

    /// Foreground text colour.
    pub text: Colour,
    pub accent: Colour,
}

impl ColourPallet {
    pub fn default_dark() -> Self {
        Self {
            primary: Colour::from_hex(0x0d0d0d),
            secondary: Colour::from_hex(0x201f1f),
            surface: Colour::from_hex(0x494848),
            text: Colour::from_hex(0xb5b5b5),
            accent: Colour::from_hex(DEFAULT_ACCENT_HEX)
        }
    }

    pub fn default_light() -> Self {
        Self {
            primary: Colour::from_hex(0xdff5f5),
            secondary: Colour::from_hex(0xd9f0ff),
            surface: Colour::from_hex(0x57575b),
            text: Colour::from_hex(0x3b3b3b),
            accent: Colour::from_hex(DEFAULT_ACCENT_HEX)
        }
    }
}