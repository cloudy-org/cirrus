use egui::{style::{Selection, Widgets}, Color32, CornerRadius, Shadow, Stroke, Visuals};

use crate::v1::styling::Styling;

impl Styling<'_> {
    pub fn set_visuals(&mut self) -> &mut Self {
        let is_dark = self.theme.pallet.is_dark;
        let colour_pallet = &self.theme.pallet;

        let primary_colour = Color32::from_hex(&colour_pallet.primary.as_hex_string()).unwrap();
        let surface_colour = Color32::from_hex(&colour_pallet.surface.as_hex_string()).unwrap();
        let text_colour = Color32::from_hex(&colour_pallet.text.as_hex_string()).unwrap();
        let accent_colour = Color32::from_hex(&colour_pallet.accent.as_hex_string()).unwrap();
    
        let secondary_accent_colour = primary_colour.blend(
            surface_colour.gamma_multiply(0.7)
        );
    
        // Derive stroke colour from subtle text colour.
        let custom_stroke_colour = text_colour.gamma_multiply(0.4);
    
        let default_visuals = match is_dark {
            true => Visuals::dark(),
            false => Visuals::light()
        };
    
        let mut widgets = match is_dark {
            true => Widgets::dark(),
            false => Widgets::light()
        };
    
        widgets.inactive.bg_fill = secondary_accent_colour;
        widgets.active.bg_fill = secondary_accent_colour;
        widgets.hovered.bg_fill = secondary_accent_colour;
    
        // this is like the outer frame that appears 
        // on some widgets when you hover on top of it.
        widgets.hovered.bg_stroke.color = custom_stroke_colour.gamma_multiply(1.2);
    
        // button rounded edge
        widgets.inactive.corner_radius = CornerRadius::same(7);
        widgets.active.corner_radius = CornerRadius::same(7);
        widgets.hovered.corner_radius = CornerRadius::same(7);
        widgets.noninteractive.corner_radius = CornerRadius::same(7);
        widgets.open.corner_radius = CornerRadius::same(7);
    
        self.egui_style.visuals = Visuals { 
            dark_mode: is_dark,
            override_text_color: Some(text_colour),
            widgets: widgets,
            // Some widgets version of "visuals.widgets.active" is "visuals.selection" 
            // like with the custom toggle button in cirrus hence this change is so the 
            // stroke is more visible when the toggle button widget is active.
            selection: Selection {
                bg_fill: accent_colour.gamma_multiply(0.95),
                stroke: Stroke { width: 2.0, color: custom_stroke_colour },
            },
            window_shadow: Shadow::NONE,
            window_fill: primary_colour.blend(surface_colour.gamma_multiply(0.4)),
            window_stroke: Stroke::new(1.0, surface_colour),
            window_highlight_topmost: false,
            panel_fill: primary_colour,
            slider_trailing_fill: true,
            ..default_visuals
        };

        self
    }
}