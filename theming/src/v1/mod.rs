#[derive(Clone)]
pub struct Colour {
    pub hex_code: String
}

#[derive(Clone)]
pub struct Theme {
    pub is_dark: bool,
    /// NOTE: The primary colour should be used as a background colour.
    pub primary_colour: Colour,
    pub secondary_colour: Colour,
    pub third_colour: Colour,
    pub accent_colour: Option<Colour>,
}

impl Theme {
    pub fn default(is_dark: bool) -> Self {
        let primary = if is_dark {
            Colour {hex_code: "#0a0909".into()}
        } else {
            Colour {hex_code: "#b4dede".into()}
        };

        let secondary = if is_dark {
            Colour {hex_code: "#201f1f".into()}
        } else {
            Colour {hex_code: "#aec5d4".into()}
        };

        let third = if is_dark {
            Colour {hex_code: "#494848".into()}
        } else {
            Colour {hex_code: "#57575b".into()}
        };

        Theme { 
            is_dark,
            primary_colour: primary,
            secondary_colour: secondary,
            third_colour: third,
            accent_colour: None,
        }
    }
}

// Retrieves the system theme... (WIP!)
// pub fn get_theme() -> Theme {}