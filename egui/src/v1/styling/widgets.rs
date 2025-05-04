use cirrus_theming::v1::Colour;
use egui::{Color32, Rounding, Style};

pub fn set_widgets_style<'a>(
    style: &'a mut Style,
    secondary_colour: &'a Colour,
    accent_colour: &'a Colour,
) {
    // give sliders a trailing colour.
    style.visuals.slider_trailing_fill = true;

    style.visuals.selection.bg_fill = Color32::from_hex(
        &accent_colour.hex_code
    ).unwrap();

    style.visuals.widgets.inactive.bg_fill = Color32::from_hex(
        &secondary_colour.hex_code
    ).unwrap();

    style.visuals.widgets.active.bg_fill = Color32::from_hex(
        &secondary_colour.hex_code
    ).unwrap();

    style.visuals.widgets.hovered.bg_fill = Color32::from_hex(
        &secondary_colour.hex_code
    ).unwrap();

    // button rounded edge
    style.visuals.widgets.inactive.rounding = Rounding::same(7.0);
    style.visuals.widgets.active.rounding = Rounding::same(7.0);
    style.visuals.widgets.hovered.rounding = Rounding::same(7.0);
    style.visuals.widgets.noninteractive.rounding = Rounding::same(7.0);
    style.visuals.widgets.open.rounding = Rounding::same(7.0);
}