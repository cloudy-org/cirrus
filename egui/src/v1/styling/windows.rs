use cirrus_theming::v1::colour::Colour;
use egui::{Color32, Shadow, Stroke, Style};

pub fn set_windows_style<'a>(
    style: &'a mut Style,
    primary_colour: &'a Colour,
    secondary_colour: &'a Colour,
    surface_colour: &'a Colour,
) {
    // Window styling.
    style.visuals.window_highlight_topmost = false;

    style.visuals.window_fill = Color32::from_hex(&secondary_colour.as_hex_string()).unwrap();
    style.visuals.window_stroke = Stroke::new(
        1.0,
        Color32::from_hex(&surface_colour.as_hex_string()).unwrap()
    );
    style.visuals.window_shadow = Shadow::NONE;

    style.visuals.widgets.inactive.bg_fill = Color32::from_hex(&primary_colour.as_hex_string()).unwrap();
}