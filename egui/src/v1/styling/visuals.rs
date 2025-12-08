use egui::{Color32, CornerRadius, Shadow, Stroke, Visuals, style::{HandleShape, Selection, Widgets}};

use crate::v1::styling::Styling;

impl Styling<'_> {
    pub fn set_visuals(&mut self) -> &mut Self {
        let is_dark = self.theme.pallet.is_dark;
        let colour_pallet = &self.theme.pallet;

        let primary_colour = Color32::from_hex(&colour_pallet.primary.to_hex_string()).unwrap();
        let interactive_colour = Color32::from_hex(&colour_pallet.interactive.to_hex_string()).unwrap();
        let surface_colour = Color32::from_hex(&colour_pallet.surface.to_hex_string()).unwrap();
        let text_colour = Color32::from_hex(&colour_pallet.text.to_hex_string()).unwrap();
        let accent_colour = Color32::from_hex(&colour_pallet.accent.to_hex_string()).unwrap();

        // Derive stroke colour from subtle text colour and put surface colour behind it.
        let stroke_colour = surface_colour.blend(
            text_colour.gamma_multiply(0.9)
        );

        // A fainter stroke colour and one that does not rely 
        // on surface colour for separators and some other widgets.
        let separator_stroke_colour = text_colour.gamma_multiply(0.3);

        let default_visuals = match is_dark {
            true => Visuals::dark(),
            false => Visuals::light()
        };

        let mut widgets = match is_dark {
            true => Widgets::dark(),
            false => Widgets::light()
        };

        widgets.inactive.bg_fill = interactive_colour;
        widgets.inactive.weak_bg_fill = interactive_colour.gamma_multiply(0.8); // used by buttons
        widgets.inactive.fg_stroke.color = stroke_colour;

        widgets.noninteractive.bg_fill = interactive_colour;
        widgets.noninteractive.weak_bg_fill = interactive_colour.gamma_multiply(0.8);
        // this bg_stroke is used by ui.separator().
        widgets.noninteractive.bg_stroke.color = separator_stroke_colour;
        widgets.noninteractive.fg_stroke.color = stroke_colour;

        widgets.hovered.bg_fill = interactive_colour;
        widgets.hovered.weak_bg_fill = interactive_colour.gamma_multiply(0.9);
        // this is like the outer frame that appears 
        // on some widgets when you hover on top of it.
        widgets.hovered.bg_stroke.color = stroke_colour;
        widgets.hovered.fg_stroke.color = stroke_colour;

        widgets.active.bg_fill = interactive_colour;
        widgets.active.weak_bg_fill = interactive_colour.gamma_multiply(0.7);
        // active.fg_stroke is used as the '.strong()' variant of text_colour 
        // so it must be very bright for dark mode themes and very dark for light mode.
        widgets.active.fg_stroke.color = match is_dark {
            true => text_colour.gamma_multiply(2.0),
            false => Color32::BLACK.blend(text_colour.gamma_multiply(0.5)),
        };

        widgets.open.bg_fill = interactive_colour;
        widgets.open.weak_bg_fill = interactive_colour.gamma_multiply(0.8); // used by combo box
        widgets.open.fg_stroke.color = stroke_colour;

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
                stroke: Stroke { width: 2.0, color: stroke_colour },
            },
            window_shadow: Shadow::NONE,
            window_fill: primary_colour.blend(surface_colour.gamma_multiply(0.4)),
            window_stroke: Stroke::new(1.0, separator_stroke_colour),
            window_highlight_topmost: false,
            panel_fill: primary_colour,
            slider_trailing_fill: true,
            faint_bg_color: interactive_colour.gamma_multiply(0.7),
            extreme_bg_color: interactive_colour,
            text_edit_bg_color: Some(primary_colour),
            handle_shape: HandleShape::Circle,
            hyperlink_color: accent_colour,
            ..default_visuals
        };

        self
    }
}