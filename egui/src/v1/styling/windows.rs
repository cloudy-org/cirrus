use cirrus_theming::v1::Colour;

use egui::{Color32, Shadow, Stroke, Style};

pub fn set_windows_style<'a>(
    style: &'a mut Style,
    primary_colour: &'a Colour,
    secondary_colour: &'a Colour,
    third_colour: &'a Colour,
) {
    // Window styling.
    style.visuals.window_highlight_topmost = false;

    style.visuals.window_fill = Color32::from_hex(
        &secondary_colour.hex_code
    ).unwrap();
    style.visuals.window_stroke = Stroke::new(
        1.0,
        Color32::from_hex(&third_colour.hex_code).unwrap()
    );
    style.visuals.window_shadow = Shadow::NONE;

    style.visuals.widgets.inactive.bg_fill =
        Color32::from_hex(
            &primary_colour.hex_code
        ).unwrap();
}