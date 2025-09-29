use cirrus_theming::v1::colour::Colour;
use egui::{Color32, CornerRadius, Style};

pub fn set_widgets_style<'a>(
    style: &'a mut Style,
    primary_colour: &'a Colour,
    surface_colour: &'a Colour,
    text_colour: &'a Colour,
    accent_colour: &'a Colour,
) {
    // give sliders a trailing colour.
    style.visuals.slider_trailing_fill = true;

    let primary_colour = Color32::from_hex(&primary_colour.as_hex_string()).unwrap();
    let surface_colour = Color32::from_hex(&surface_colour.as_hex_string()).unwrap();
    let text_colour = Color32::from_hex(&text_colour.as_hex_string()).unwrap();
    let accent_colour = Color32::from_hex(&accent_colour.as_hex_string()).unwrap();

    let secondary_accent_colour = primary_colour.blend(
        surface_colour.gamma_multiply(0.7)
    );

    // accent colour for widget's background fills should be darker
    style.visuals.selection.bg_fill = accent_colour.gamma_multiply(0.95);

    style.visuals.widgets.inactive.bg_fill = secondary_accent_colour;
    style.visuals.widgets.active.bg_fill = secondary_accent_colour;
    style.visuals.widgets.hovered.bg_fill = secondary_accent_colour;

    // Improvements to widget frames and strokes.
    let custom_stroke_colour = text_colour.gamma_multiply(0.4);

    // Some widgets version of "visuals.widgets.active" is "visuals.selection" 
    // like with the custom toggle button in cirrus hence this change is so the 
    // stroke is more visible when the toggle button widget is active.
    let active_selection_stroke = &mut style.visuals.selection.stroke;

    active_selection_stroke.width = 2.0;
    active_selection_stroke.color = custom_stroke_colour;

    // this is like the outer frame that appears 
    // on some widgets when you hover on top of it.
    style.visuals.widgets.hovered.bg_stroke.color = custom_stroke_colour.gamma_multiply(1.2);

    // button rounded edge
    style.visuals.widgets.inactive.corner_radius = CornerRadius::same(7);
    style.visuals.widgets.active.corner_radius = CornerRadius::same(7);
    style.visuals.widgets.hovered.corner_radius = CornerRadius::same(7);
    style.visuals.widgets.noninteractive.corner_radius = CornerRadius::same(7);
    style.visuals.widgets.open.corner_radius = CornerRadius::same(7);
}