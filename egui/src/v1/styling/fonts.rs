use cirrus_theming::v1::Colour;

use egui::{Color32, Style, TextStyle};

pub fn set_font_style<'a>(style: &'a mut Style, text_style: TextStyle, text_colour: &'a Colour) {
    style.override_text_style = Some(text_style);

    style.visuals.override_text_color = Some(
        Color32::from_hex(&text_colour.hex_code).unwrap()
    );
}