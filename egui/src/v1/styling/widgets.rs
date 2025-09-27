use cirrus_theming::v1::Colour;
use egui::{Color32, Rounding, Style};

pub fn set_widgets_style<'a>(
    style: &'a mut Style,
    secondary_colour: &'a Colour,
    accent_colour: &'a Colour,
) {
    // give sliders a trailing colour.
    style.visuals.slider_trailing_fill = true;

    let accent_colour = Color32::from_hex(&accent_colour.hex_code).unwrap();

    // accent colour for widget's background fills should be darker
    style.visuals.selection.bg_fill = accent_colour.gamma_multiply(0.95);

    style.visuals.widgets.inactive.bg_fill = Color32::from_hex(
        &secondary_colour.hex_code
    ).unwrap();

    style.visuals.widgets.active.bg_fill = Color32::from_hex(
        &secondary_colour.hex_code
    ).unwrap();

    style.visuals.widgets.hovered.bg_fill = Color32::from_hex(
        &secondary_colour.hex_code
    ).unwrap();

    // Improvements to widget frames and strokes.

    // Some widgets version of "visuals.widgets.active" is "visuals.selection" 
    // like with the custom toggle button in cirrus hence this change is so the 
    // stroke is more visible when the toggle button widget is active.
    let active_selection_stroke = &mut style.visuals.selection.stroke;

    active_selection_stroke.width = 2.0;
    active_selection_stroke.color = active_selection_stroke.color.gamma_multiply(1.2);

    // this is like the outer frame that appears 
    // on some widgets when you hover on top of it.
    style.visuals.widgets.hovered.bg_stroke.color = style.visuals.widgets.hovered.bg_stroke.color
        .gamma_multiply(1.5);

    // button rounded edge
    style.visuals.widgets.inactive.rounding = Rounding::same(7.0);
    style.visuals.widgets.active.rounding = Rounding::same(7.0);
    style.visuals.widgets.hovered.rounding = Rounding::same(7.0);
    style.visuals.widgets.noninteractive.rounding = Rounding::same(7.0);
    style.visuals.widgets.open.rounding = Rounding::same(7.0);
}