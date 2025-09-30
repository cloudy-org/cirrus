use cirrus_theming::v1::theme::Theme;
use egui::{Context, Style, TextStyle};

pub mod fonts;
pub(crate) mod visuals;

/// Cirrus handled styling for egui... so you don't have to do allat.
pub struct Styling<'a> {
    theme: &'a Theme,
    pub egui_style: Style,
}

impl<'a> Styling<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        let style = Style {
            override_text_style: Some(TextStyle::Monospace),
            ..Default::default()
        };

        Self {
            theme,
            egui_style: style
        }
    }

    /// Style the entirely of egui to cloudy org styling.
    pub fn set_all(&mut self) -> &Self {
        self
            .set_visuals()
            .set_fonts(None)
    }

    pub fn apply(&self, ctx: &Context) {
        ctx.set_style(self.egui_style.clone());
    }
}