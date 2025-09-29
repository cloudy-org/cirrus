use cirrus_theming::v1::colour::Colour;
use egui::{Color32, Shadow, Stroke, Style};

pub fn set_windows_style<'a>(
    style: &'a mut Style,
    primary_colour: &'a Colour,
    surface_colour: &'a Colour,
) {
    let primary_colour = Color32::from_hex(&primary_colour.as_hex_string()).unwrap();
    let surface_colour = Color32::from_hex(&surface_colour.as_hex_string()).unwrap();

    // Window styling.
    style.visuals.window_highlight_topmost = false;

    style.visuals.window_fill = primary_colour.blend(surface_colour.gamma_multiply(0.4));
    style.visuals.window_stroke = Stroke::new(
        1.0, surface_colour
    );
    style.visuals.window_shadow = Shadow::NONE;

    style.visuals.widgets.inactive.bg_fill = primary_colour;
}