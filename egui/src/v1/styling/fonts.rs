use egui::{TextStyle};

use crate::v1::styling::Styling;

impl Styling<'_> {
    pub fn set_fonts(&mut self, text_style: Option<TextStyle>) -> &mut Self {
        let text_style = match text_style {
            Some(text_style) => text_style,
            None => TextStyle::Monospace,
        };

        self.egui_style.override_text_style = Some(text_style);

        self
    }
}