use cirrus_theming::v1::Theme;

use egui::{Context, Style, TextStyle};

pub mod fonts;
pub mod windows;
pub mod background;

/// Cirrus handled styling for egui... so you don't have to do allat.
pub struct Styling<'a> {
    theme: &'a Theme,
    pub egui_style: Style,
}

impl<'a> Styling<'a> {
    pub fn new(theme: &'a Theme, custom_style: Option<Style>) -> Self {
        let style = match custom_style {
            Some(style) => style,
            None => Style {
                override_text_style: Some(TextStyle::Monospace),
                ..Default::default()
            },
        };

        Self {
            theme,
            egui_style: style
        }
    }

    /// Style the entirely of egui to cloudy org styling.
    pub fn set_all(&mut self) -> &Self {
        self
            .set_background()
            .set_windows()
            .set_fonts(None)
    }

    pub fn apply(&self, ctx: &Context) {
        ctx.set_style(self.egui_style.clone());
    }
}

impl Styling<'_> {
    pub fn set_fonts(&mut self, text_style: Option<TextStyle>) -> &mut Self {
        let text_style = match text_style {
            Some(text_style) => text_style,
            None => TextStyle::Monospace,
        };

        fonts::set_font_style(&mut self.egui_style, text_style, &self.theme.text_colour);

        self
    }

    pub fn set_background(&mut self) -> &mut Self {
        background::set_background_style(&mut self.egui_style, &self.theme.primary_colour);

        self
    }

    pub fn set_windows(&mut self) -> &mut Self {
        windows::set_windows_style(
            &mut self.egui_style,
            &self.theme.primary_colour,
            &self.theme.secondary_colour,
            &self.theme.third_colour
        );

        self
    }
}