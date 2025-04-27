use egui::{Style, Visuals};

/// MUST be set FIRST.
pub fn set_base_visuals_style<'a>(
    style: &'a mut Style,
    is_dark: bool
) {
    style.visuals = match is_dark {
        true => Visuals::dark(),
        false => Visuals::light()
    };
}