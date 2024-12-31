use cirrus_theming::v1::Colour;

use egui::{Color32, Style};

pub fn set_background_style<'a>(style: &'a mut Style, primary_colour: &'a Colour) {
    // Background colour styling.
    style.visuals.panel_fill = Color32::from_hex(
        &primary_colour.hex_code
    ).unwrap();
}