use cirrus_theming::v1::Theme;
use egui::{Color32, Context, CornerRadius, Frame, Margin, RichText, Stroke, Ui};

pub struct Settings {}

impl Settings {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ctx: &Context, ui: &mut Ui, theme: &Theme) {
        ui.vertical_centered(|ui| {
            let grid = Frame::group(&ctx.style())
                .outer_margin(Margin::same(7))
                .stroke(Stroke::NONE)
                .fill(Color32::from_hex(&theme.secondary_colour.hex_code).unwrap())
                .corner_radius(CornerRadius::same(20));

            grid.show(ui, |ui| {
                ui.heading(RichText::new("Settings").strong().size(70.0));
                ui.add_space(5.0);
                ui.end_row();

                // TODO: generate setting fields here deriving from inputted config.template.toml.

                ui.label("Second row, first column");
                ui.label("Second row, second column");
                ui.label("Second row, third column");
                ui.end_row();
            });
        });

    }
}