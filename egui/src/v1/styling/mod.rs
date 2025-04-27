use cirrus_theming::v1::Theme;

use egui::{Context, Style, TextStyle};

pub mod fonts;
pub mod windows;
pub mod background;
pub(crate) mod visuals;
pub(crate) mod widgets;

/// Cirrus handled styling for egui... so you don't have to do allat.
/// 
/// **WARNING:** `custom_style` does not override anything that is being set in `Styling`!
/// For example attributes like `.visuals` will not be overridden with `custom_style` if 
/// `Styling.set_base_visuals()` is invoked. (`.set_base_visuals()` is invoked in `.set_all()`)
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
            .set_base_visuals()
            .set_background()
            .set_windows()
            .set_widgets()
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

    /// MUST be set FIRST.
    pub fn set_base_visuals(&mut self) -> &mut Self {
        visuals::set_base_visuals_style(&mut self.egui_style, self.theme.is_dark);

        self
    }

    pub fn set_widgets(&mut self) -> &mut Self {
        widgets::set_widgets_style(
            &mut self.egui_style,
            &self.theme.secondary_colour,
            &self.theme.accent_colour
        );

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