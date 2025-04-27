#[derive(Clone)]
pub struct Colour {
    pub hex_code: String
}

impl Colour {
    pub fn from_hex(hex_code: &str) -> Self {
        Self { hex_code: hex_code.to_owned() }
    }
}

#[derive(Clone)]
pub struct Theme {
    pub is_dark: bool,
    /// Primary colour should mostly be used for background colour.
    pub primary_colour: Colour,
    /// Secondary colour is mostly used as a secondary background colour for floating windows 
    /// or widgets but it's a **secondary** colour so you may use it wherever you prefer is best.
    pub secondary_colour: Colour,
    /// The complete usage of this colour is up to you for now. In Roseate I've used it for floating window outlines.
    pub third_colour: Colour,
    pub text_colour: Colour,
    pub accent_colour: Colour,
}

impl Theme {
    /// Create a new theme with your own colours and fallback to some good default ones.
    /// 
    /// If you want to just use default colours, set an empty 
    /// array in `colours` parameter and leave `accent_colour` as `None`.
    pub fn new(
        is_dark: bool,
        colours: Vec<Colour>,
        accent_colour: Option<Colour>
    ) -> Self {
        #![allow(warnings)] // default is deprecated but only deprecated for public use.
        let default_theme = Self::default(is_dark);

        Self {
            is_dark,
            primary_colour: colours.get(0).unwrap_or(&default_theme.primary_colour).to_owned(),
            secondary_colour: colours.get(1).unwrap_or(&default_theme.secondary_colour).to_owned(),
            third_colour: colours.get(2).unwrap_or(&default_theme.third_colour).to_owned(),
            text_colour: default_theme.text_colour,
            accent_colour: accent_colour.unwrap_or(default_theme.accent_colour),
        }
    }

    #[deprecated(note="`Theme::default` may become private in future commits of cirrus, I would recommend the use of `Theme::new` instead.")]
    pub fn default(is_dark: bool) -> Self {
        let primary = if is_dark {
            Colour {hex_code: "#0d0d0d".into()}
        } else {
            Colour {hex_code: "#dff5f5".into()}
        };

        let secondary = if is_dark {
            Colour {hex_code: "#201f1f".into()}
        } else {
            Colour {hex_code: "#d9f0ff".into()}
        };

        let third = if is_dark {
            Colour {hex_code: "#494848".into()}
        } else {
            Colour {hex_code: "#57575b".into()}
        };

        let text_colour = Colour::from_hex(
            match is_dark {
                true => "#b5b5b5",
                false => "#3b3b3b",
            }.into()
        );

        Theme { 
            is_dark,
            primary_colour: primary,
            secondary_colour: secondary,
            third_colour: third,
            text_colour,
            accent_colour: Colour::from_hex("#7afff8"),
        }
    }
}

// Retrieves the system theme... (WIP!)
// pub fn get_theme() -> Theme {}