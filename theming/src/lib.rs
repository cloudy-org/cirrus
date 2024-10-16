
pub struct Theme {
    pub is_dark: bool,
    pub hex_code: String
}

impl Theme {
    pub fn default(is_dark: bool) -> Self {
        let hex_code = if is_dark {
            String::from("#0a0909")
        } else {
            String::from("#b4dede")
        };

        Theme { 
            is_dark, hex_code
        }
    }
}

// Retrieves the system theme... (WIP!)
// pub fn get_theme() -> Theme {}